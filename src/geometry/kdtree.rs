use std::sync::Arc;
use cgmath::{Vector3, Zero, ElementWise, Array};
use std::cmp::{max, min, Ordering};
use crate::geometry::kdtree::EdgeType::{ET_start, ET_end};
use arrayvec::ArrayVec;
use std::f32::INFINITY;
use std::fmt::{Debug, Formatter};
use std::fmt;
use cgmath::num_traits::Inv;
use std::ptr::null;
use std::borrow::Borrow;
use obj::Obj;
use std::collections::hash_map::DefaultHasher;

use crate::world::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::geometry::trimesh::{TriMesh, MeshTriangle};
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::geometry::{Boundable, Geometry, GeomError, BoundedConcrete, Shadable};
use crate::geometry::bbox::BBox;

// TODO: Fix fatal bugs in this implementation
/// KDTree is implemented for accelerating ray tracing. The implementation takes reference from
/// "Physically Based Rendering: From Theory To Implementation" by Matt Pharr, Wenzel Jakob, and Greg Humphreys.
pub struct KDTree<T> where T: BoundedConcrete + Clone
{
    pub m_primitives: Vec<T>,
    pub m_sorted_indices: Vec<u32>,

    pub m_max_prim_per_node: u8,
    pub m_max_depth: u32,
    pub m_bounds: BBox,

    // Constants      ///////////
    m_empty_bonus: f32,
    m_intersect_cost: f32,
    m_traversal_cost: f32,

    m_number_of_allocated_nodes: u32,
    m_next_free_node_slot: u32,
    m_nodes: Vec<KDTreeNode>,

    // field needed for shading
    m_material: Option<Arc<dyn Material>>,
}

impl<T> KDTree<T> where T: BoundedConcrete + Clone
{
    const MAX_KDTREE_TASKS: u8 = 64;

    pub fn new(prim_vec: Vec<T>,
                intersect_cost: f32,
                traversal_cost: f32,
                empty_bonus: f32,
                max_prim_per_node: u8,
                max_depth: u32) -> KDTree<T>
    {
        let prim_vec_len = prim_vec.len();
        KDTree
        {
            m_primitives: prim_vec.clone(),
            m_sorted_indices: Vec::with_capacity(prim_vec_len),

            m_max_prim_per_node: max_prim_per_node,
            m_max_depth: if max_depth == 0 { (8.0 + (1.3 * prim_vec_len as f32).floor()) as u32 }
                        else { max_depth },
            m_bounds: BBox::new(Vector3::zero(), Vector3::zero()),

            m_empty_bonus: empty_bonus,
            m_intersect_cost: intersect_cost,
            m_traversal_cost: traversal_cost,

            m_number_of_allocated_nodes: 0,
            m_next_free_node_slot: 0,
            m_nodes: Vec::with_capacity(0),
            m_material: None,
        }
    }

    pub fn init(&mut self)
    {
        let mut vec_bbox = Vec::with_capacity(self.m_primitives.len());

        for primitive in self.m_primitives.iter()
        {
            let b = primitive.get_bbox();
            self.m_bounds = self.m_bounds.union(&b);
            vec_bbox.push(b);
        }

        // Allocate memory for kd-tree construction
        let mut edges_vec = vec![vec![ BoundEdge::default(); 2 * self.m_primitives.len()]; 3];
        let mut edges = edges_vec.iter_mut().map(|v| v.as_mut_slice()).collect();

        let mut prims0_vec = vec![0_u16; self.m_primitives.len()];
        let prims0 = prims0_vec.as_mut_slice();

        let mut prims1_vec = vec![0_u16; (self.m_max_depth + 1) as usize * self.m_primitives.len()];
        let prims1 = prims1_vec.as_mut_slice();

        let mut arr_prim_nums: Vec<u32> = (0..self.m_primitives.len() as u32).collect();
        let prim_nums = arr_prim_nums.as_mut_slice();

        let bounds = self.m_bounds.clone();
        let n_primitives = self.m_primitives.len() as u32;
        let max_depth = self.m_max_depth.clone();
        KDTree::built_tree(self, 0, &bounds, &vec_bbox,
                        prim_nums, n_primitives, max_depth,
                        &mut edges, prims0, prims1, 0);
    }

    /**
     * This method builds the KD-tree recursively.
     */
    fn built_tree(&mut self, node_num: usize, node_bbox: &BBox, all_prim_bbox: &Vec<BBox>,
                  prim_nums: &mut [u32], n_primitives: u32, depth: u32,
                  edges_ref: &mut Vec<&mut [BoundEdge]>, prims0: &mut [u16], prims1: &mut [u16], mut bad_refine: u8)
    {
        assert_eq!(node_num as u32, self.m_next_free_node_slot);

        // Allocate more memory in case we ran out of node slots.
        if self.m_next_free_node_slot == self.m_number_of_allocated_nodes
        {
            let additional_allocated_nodes = max(2 * self.m_number_of_allocated_nodes, 512) as usize - self.m_nodes.len();
            let new_len = additional_allocated_nodes + self.m_nodes.len();
            self.m_nodes.resize(new_len, DEFAULT_NODE);
            self.m_number_of_allocated_nodes = self.m_nodes.len() as u32;
        }

        self.m_next_free_node_slot += 1;

        // Initialize leaf node if termination criteria is met
        if n_primitives <= self.m_max_prim_per_node as u32 || depth == 0
        {
            self.m_nodes[node_num].create_leaf(prim_nums, n_primitives as usize, &mut self.m_sorted_indices);
            return;
        }

        // Else, initialize interior node and continue
        let mut best_axis = 4;
        let mut best_offset = 0;
        let mut best_cost = INFINITY;
        let old_cost = self.m_intersect_cost * n_primitives as f32;
        let inv_total_SA = node_bbox.get_surface_area().inv();
        let diff = node_bbox.get_diagonal();

        // Choose the axis to split
        let mut axis = node_bbox.maximum_extent() as usize;
        let mut retries = 0;

        loop
        {
            for i in 0..n_primitives as usize
            {
                let _prim_num = prim_nums[i];
                let bbox = all_prim_bbox[_prim_num as usize].clone();

                unsafe
                    {
                        edges_ref[axis][2*i] = BoundEdge::new(bbox.m_vertex_0[axis as usize], _prim_num, true);
                        edges_ref[axis][2*i + 1] = BoundEdge::new(bbox.m_vertex_1[axis as usize], _prim_num, false);
                    }
            }

            // Sort the edges.
            edges_ref[axis].sort_by(KDTree::<T>::edge_sort);

            // Compute costs of all splits for axis to find the best cost.
            let mut n_below = 0 as usize;
            let mut n_above = n_primitives;

            for i in 0..(2 * n_primitives) as usize
            {
                if edges_ref[axis][i].m_type == EdgeType::ET_end
                {
                    n_above -= 1;
                }
                let edgeT = edges_ref[axis][i].m_t;
                if edgeT > KDTree::<T>::vector3_index_get(&node_bbox.m_vertex_0, axis as u8)
                    && edgeT < KDTree::<T>::vector3_index_get(&node_bbox.m_vertex_1, axis as u8)
                {
                    // Compute cost for aplit at this edge.
                    // 1. Compute child surface areas.
                    let axis0 = (axis + 1) % 3;
                    let axis1 = (axis + 2) % 3;

                    let below_surfacearea = 2.0 * (diff[axis0] * diff[axis1]
                        + (edgeT - node_bbox.m_vertex_0[axis]) * (diff[axis0] + diff[axis1]));
                    let above_surfacearea = 2.0 * (diff[axis0] * diff[axis1]
                        + (node_bbox.m_vertex_1[axis] - edgeT) * (diff[axis0] + diff[axis1]));

                    let percentage_below = below_surfacearea * inv_total_SA;
                    let percentage_above = above_surfacearea * inv_total_SA;

                    let empty_bonus = if n_above == 0 || n_below == 0 { self.m_empty_bonus } else { 0.0 };
                    let cost = self.m_traversal_cost +
                        self.m_intersect_cost * (1.0 - empty_bonus) * (n_below as f32 * percentage_below + n_above as f32 * percentage_above);

                    if cost < best_cost
                    {
                        best_axis = axis;
                        best_cost = cost;
                        best_offset = i;
                    }
                }
                if edges_ref[axis][i].m_type == EdgeType::ET_start
                {
                    n_below += 1;
                }
            }

            // If we can't find a good split on this axis, try another axis.
            if best_axis == 4 && retries < 2
            {
                axis = (axis + 1) % 3;
                retries += 1;
                // TODO a goto statement in c++ code!!!
            }
            else { break; }
        }


        bad_refine += if best_cost > old_cost { 1 } else { 0 };
        if (best_cost > 4.0 * old_cost && n_primitives < 16) || best_axis == 4 || bad_refine == 3
        {
            self.m_nodes[node_num as usize].create_leaf(prim_nums, n_primitives as usize, &mut self.m_sorted_indices);
        }

        // Classify primitives wrt the split.else
        let mut n0 = 0;
        let mut n1 = 0;
        for i in 0..best_offset
        {
            if edges_ref[best_axis][i].m_type == EdgeType::ET_start
            {
                prims0[n0] = edges_ref[best_axis][i].m_prim_num as u16;
                n0 += 1;
            }
        }
        for i in (best_offset + 1)..2*n_primitives as usize
        {
            if edges_ref[best_axis][i].m_type == EdgeType::ET_start
            {
                prims0[n1] = edges_ref[best_axis][i].m_prim_num as u16;
                n1 += 1;
            }
        }

        let t_split = edges_ref[best_axis as usize][best_offset].m_t;
        let mut bbox0 = (*node_bbox).clone();
        let mut bbox1 = (*node_bbox).clone();
        KDTree::<T>::vector3_index_set(&mut bbox0.m_vertex_1, best_axis as u8, t_split);
        KDTree::<T>::vector3_index_set(&mut bbox1.m_vertex_0, best_axis as u8, t_split);

        self.built_tree(node_num + 1, &bbox0, all_prim_bbox,
                        prim_nums, n0 as u32, depth - 1, edges_ref,
                        prims0, &mut prims1[n_primitives as usize..], bad_refine);

        let above_child = self.m_next_free_node_slot as usize;
        self.m_nodes[node_num].create_interior(best_axis as u8, above_child as u32, t_split);

        self.built_tree(above_child, &bbox1, all_prim_bbox,
                        prim_nums, n1 as u32, depth + 1, edges_ref,
                        prims0, &mut prims1[n_primitives as usize..], bad_refine);

    }

    fn edge_sort(lhs: &BoundEdge, rhs: &BoundEdge) -> std::cmp::Ordering
    {
        return if lhs.m_t == rhs.m_t
        {
            if lhs.m_type < rhs.m_type { Ordering::Less } else { Ordering::Greater }
        }
        else
        {
            if lhs.m_t < rhs.m_t { Ordering::Less } else { Ordering::Greater }
        };
    }

    pub fn vector3_index_get(vector: &Vector3<f32>, index: u8) -> f32
    {
        match index
        {
            0 => vector.x,
            1 => vector.y,
            2 => vector.z,
            _ => panic!("Index out of range for Vector3."),
        }
    }

    pub fn vector3_index_set(vector: &mut Vector3<f32>, index: u8, val: f32)
    {
        match index
        {
            0 => { vector.x = val; },
            1 => { vector.y = val; },
            2 => { vector.z = val; },
            _ => panic!("Index out of range for Vector3."),
        }
    }

    fn custom_shadow_hit(&self, shadowray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> (bool, u32)
    {
        let mut tmin = 0.0_f32;
        let mut tmax = 0.0_f32;
        if !self.m_bounds.calculate_hit_time(shadowray, &mut tmin, &mut tmax)
        {
            return (false, 0);
        }

        let inv_dir = Vector3::new(shadowray.m_direction.x.inv(),
                                   shadowray.m_direction.y.inv(),
                                   shadowray.m_direction.z.inv());

        let mut tasks = vec![KDTasks::default(); 64];
        let mut task_offset = 0;
        let mut result: (bool, u32) = (false, 0);
        let mut node = self.m_nodes.as_ptr();
        unsafe
            {
                while let node_ref = node.as_ref().unwrap()
                {
                    if node_ref.is_leaf()
                    {
                        match node_ref.get_n_primitives()
                        {
                            1 =>
                                {
                                    let index = node_ref.m_pub_union.m_one_primitive;
                                    if self.m_primitives[index as usize].shadow_hit(shadowray, time)
                                    {
                                        result = (true, index as u32);
                                    }
                                }
                            _ =>
                                {
                                    let mut time_temp = INFINITY;
                                    for i in node_ref.get_primitives_indices_offset() as usize..min(node_ref.get_n_primitives() as usize, self.m_sorted_indices.len())
                                    {
                                        let index = self.m_sorted_indices[i];
                                        if self.m_primitives[index as usize].shadow_hit(shadowray, &mut time_temp)
                                        {
                                            result = (true, index);
                                        }
                                    }
                                }
                        }

                        if task_offset > 0
                        {
                            task_offset -= 1;
                            node = tasks[task_offset].m_node;
                            tmin = tasks[task_offset].m_tmin;
                            tmax = tasks[task_offset].m_tmax;
                        } else { break; }
                    }
                    else
                    {
                        let axis = node_ref.get_split_axis();
                        let t_plane = KDTree::<T>::vector3_index_get(&inv_dir,axis) * (node_ref.get_split_position() - KDTree::<T>::vector3_index_get(&shadowray.m_origin, axis));

                        let mut first_child = null();
                        let mut second_child = null();
                        match KDTree::<T>::vector3_index_get(&shadowray.m_direction, axis) < node_ref.get_split_position()
                            || (KDTree::<T>::vector3_index_get(&shadowray.m_direction, axis) == node_ref.get_split_position()
                            && KDTree::<T>::vector3_index_get(&shadowray.m_direction, axis) < 0.0)
                        {
                            true =>
                                unsafe {
                                    first_child = node.offset(1);
                                    second_child = self.m_nodes.as_ptr().offset((*node).m_priv_union.m_above_child as isize);
                                }
                            false =>
                                unsafe
                                    {
                                        first_child = self.m_nodes.as_ptr().offset((*node).m_priv_union.m_above_child as isize);
                                        second_child = node.offset(1);
                                    }
                        }
                        // Advance to next child node, possibly enqueue another children
                        if t_plane > tmax || t_plane <= 0.0 { node = first_child; }
                        else if t_plane < tmin { node = second_child; }
                        else
                        {
                            tasks[task_offset].m_node = second_child;
                            tasks[task_offset].m_tmin = t_plane;
                            tasks[task_offset].m_tmax = tmax;
                            task_offset += 1;
                            node = first_child;
                            tmax = t_plane;
                        }
                    }
                }
            }
        (false, 0)
    }
}

impl<T> Debug for KDTree<T> where T: BoundedConcrete + Clone
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("KDTree")
            .finish()
    }
}

impl<T> Geometry for KDTree<T> where T: BoundedConcrete + Clone
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        let mut dummy_time = 0.0_f32;
        let mut dummy_sr = ShadeRec::get_dummy();
        match self.custom_shadow_hit(incomeray, &mut dummy_time, &mut dummy_sr)
        {
            (true, index) => self.m_primitives[index as usize].hit(incomeray, time, shaderecord),
            (false, index) => Ok(false),
        }

    }
}

impl<T> Shadable for KDTree<T> where T: BoundedConcrete + Clone
{
    fn get_material(&self) -> Arc<dyn Material>
    {
        if let Some(x) = self.m_material.clone() { x }
        else { panic!("The material for sphere is Not set") }
    }

    fn set_material(&mut self, material: Arc<dyn Material>)
    {
        self.m_material = Some(material.clone());
    }

    /// Return a tuple (does_hit: bool, triangle_index: usize)
    fn shadow_hit(&self, shadowray: &Ray, tmin: &mut f32) -> bool
    {
        let mut dummy = ShadeRec::get_dummy();
        match self.custom_shadow_hit(shadowray, tmin, &mut dummy)
        {
            (true, index) => true,
            (false, index) => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum EdgeType
{
    ET_start,
    ET_end,
    ET_undefined,
}

impl Default for EdgeType
{
    fn default() -> Self { EdgeType::ET_undefined }
}

#[derive(Clone, Debug, Default)]
struct BoundEdge
{
    m_t: f32,
    m_prim_num: u32,
    m_type: EdgeType,
}

impl BoundEdge
{
    pub fn new(t: f32, prim_num: u32, starting: bool) -> BoundEdge
    {
        BoundEdge
        {
            m_t: t,
            m_prim_num: prim_num,
            m_type: if starting { ET_start } else { ET_end },
        }
    }
}

#[derive(Clone)]
struct KDTreeNode
{
    pub m_pub_union: KDTreeNode_pub_union,
    m_priv_union: KDTreeNode_priv_union,
}

const DEFAULT_NODE: KDTreeNode = KDTreeNode
{
    m_pub_union: KDTreeNode_pub_union{ m_one_primitive: 0 },
    m_priv_union: KDTreeNode_priv_union{ m_flags: 0 }
};

impl KDTreeNode
{
    fn new() -> KDTreeNode
    {
        KDTreeNode
        {
            m_pub_union: KDTreeNode_pub_union{ m_one_primitive: 0 },
            m_priv_union: KDTreeNode_priv_union{ m_flags: 0 },
        }
    }

    fn create_leaf(&mut self, prim_nums: &mut [u32], np: usize, primitive_indices: &mut Vec<u32>)
    {
        unsafe // Modify unions
            {
                self.m_priv_union.m_flags = 3;
                self.m_priv_union.m_n_prims |= prim_nums[np] << 2;

                match np
                {
                    0 => { self.m_pub_union.m_one_primitive = 0; }
                    1 => { self.m_pub_union.m_one_primitive = prim_nums[0] as u8; }
                    _ => {
                        self.m_pub_union.m_prim_indices_offset = primitive_indices.len() as u32;
                        primitive_indices.extend_from_slice(&prim_nums[0..np]);
                    }
                }
            }
    }

    fn create_interior(&mut self, axis: u8, ac: u32, s: f32)
    {
        unsafe {
            self.m_pub_union.m_split = s;
            self.m_priv_union.m_flags = axis;
            self.m_priv_union.m_above_child |= ac << 2;
        }
    }

    pub fn get_split_position(&self) -> f32 { unsafe { self.m_pub_union.m_split } }
    pub fn get_n_primitives(&self) -> u32 { unsafe { self.m_priv_union.m_n_prims >> 2 } }
    pub fn get_split_axis(&self) -> u8 { unsafe { self.m_priv_union.m_flags & 3 } }
    pub fn is_leaf(&self) -> bool { unsafe { self.m_priv_union.m_flags & 3 == 3 } }
    pub fn get_above_child(&self) -> u32 { unsafe { self.m_priv_union.m_above_child >> 2 } }
    pub fn get_primitives_indices_offset(&self) -> u32 { unsafe { self.m_pub_union.m_prim_indices_offset } }
}

#[derive(Copy, Clone)]
union KDTreeNode_pub_union
{
    m_split: f32,                 // Interior
    m_one_primitive: u8,          // Leaf
    m_prim_indices_offset: u32, // Leaf
}

#[derive(Copy, Clone)]
union KDTreeNode_priv_union
{
    m_flags: u8,
    m_n_prims: u32,
    m_above_child: u32,
}

#[derive(Clone, Debug)]
struct KDTasks
{
    m_node: *const  KDTreeNode,
    m_tmin: f32,
    m_tmax: f32,
}

impl KDTasks
{
    pub fn new() -> KDTasks
    {
        KDTasks
        {
            m_node: null(),
            m_tmin: INFINITY,
            m_tmax: -0.0,
        }
    }
}

impl Default for KDTasks
{
    fn default() -> Self {
        KDTasks::new()
    }
}