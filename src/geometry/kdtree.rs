use crate::geometry::{Boundable, Geometry, GeomError, BoundedConcrete, Shadable};
use crate::geometry::bbox::BBox;
use std::sync::Arc;
use cgmath::{Vector3, Zero, ElementWise, Array};
use std::cmp::{max, min, Ordering};
use crate::geometry::kdtree::EdgeType::{ET_start, ET_end};
use arrayvec::ArrayVec;
use std::f32::INFINITY;
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;
use std::fmt::{Debug, Formatter};
use std::fmt;
use cgmath::num_traits::Inv;
use std::ptr::null;
use std::borrow::Borrow;

/// KDTree is implemented for accelerating ray tracing. The implementation takes reference from
/// "Physically Based Rendering: From Theory To Implementation" by Matt Pharr, Wenzel Jakob, and Greg Humphreys.

pub struct KDTree<T> where T: BoundedConcrete
{
    pub m_primitives: Vec<Arc<T>>,
    pub m_sorted_indices: Vec<usize>,

    pub m_max_prim_per_node: usize,
    pub m_max_depth: usize,
    pub m_bounds: BBox,

    // Constants      ///////////
    m_empty_bonus: f32,
    m_intersect_cost: f32,
    m_traversal_cost: f32,

    m_number_of_allocated_nodes: usize,
    m_next_free_node_slot: usize,
    m_nodes: Vec<KDTreeNode>,
}

impl<T> KDTree<T> where T: BoundedConcrete
{
    const MAX_KDTREE_TASKS: u8 = 64;

    pub fn new(prim_vec: Vec<Arc<T>>,
                intersect_cost: f32,
                traversal_cost: f32,
                empty_bonus: f32,
                max_prim_per_node: usize,
                max_depth: usize) -> KDTree<T>
    {
        let prim_vec_len = prim_vec.len();
        KDTree
        {
            m_primitives: prim_vec,
            m_sorted_indices: Vec::with_capacity(prim_vec_len),

            m_max_prim_per_node: max_prim_per_node,
            m_max_depth: if max_depth <= 0 { (8.0 + (1.3 * prim_vec_len as f32).floor()) as usize }
                        else { max_depth },
            m_bounds: BBox::new(Vector3::zero(), Vector3::zero()),

            m_empty_bonus: empty_bonus,
            m_intersect_cost: intersect_cost,
            m_traversal_cost: traversal_cost,

            m_number_of_allocated_nodes: 0,
            m_next_free_node_slot: 0,
            m_nodes: Vec::with_capacity(0),
        }
    }

    pub fn init(&mut self)
    {
        let mut vec_bbox = Vec::with_capacity(self.m_primitives.len());

        for primitive in self.m_primitives.iter()
        {
            let b = primitive.as_ref().get_bbox();
            self.m_bounds = self.m_bounds.union(&b);
            vec_bbox.push(b);
        }

        // Allocate memory for kd-tree construction
        let mut edges_vec = vec![vec![ BoundEdge::default(); 2 * self.m_primitives.len()]; 3];
        let mut edges = edges_vec.iter_mut().map(|v| v.as_mut_slice()).collect();

        let mut prims0_vec = vec![0_u16; self.m_primitives.len()];
        let prims0 = prims0_vec.as_mut_slice();

        let mut prims1_vec = vec![0_u16; (self.m_max_depth + 1) * self.m_primitives.len()];
        let prims1 = prims1_vec.as_mut_slice();

        let mut arr_prim_nums: Vec<usize> = (0..self.m_primitives.len()).collect();
        let prim_nums = arr_prim_nums.as_mut_slice();

        let bounds = self.m_bounds.clone();
        let n_primitives = self.m_primitives.len();
        let max_depth = self.m_max_depth.clone();
        KDTree::built_tree(self, 0, &bounds, &vec_bbox,
                        prim_nums, n_primitives, max_depth,
                        &mut edges, prims0, prims1, 0);
    }

    /**
     * This method builds the KD-tree recursively.
     */
    fn built_tree(&mut self, node_num: usize, node_bbox: &BBox, all_prim_bbox: &Vec<BBox>,
                  prim_nums: &mut [usize], n_primitives: usize, depth: usize,
                  edges_ref: &mut Vec<&mut [BoundEdge]>, prims0: &mut [u16], prims1: &mut [u16], mut bad_refine: u8)
    {
        assert_eq!(node_num as usize, self.m_next_free_node_slot);

        // Allocate more memory in case we ran out of node slots.
        if self.m_next_free_node_slot == self.m_number_of_allocated_nodes
        {
            let additional_allocated_nodes = max(2 * self.m_number_of_allocated_nodes, 512) - self.m_nodes.len();
            self.m_nodes.reserve(additional_allocated_nodes);
            self.m_number_of_allocated_nodes = self.m_nodes.len();
        }

        self.m_next_free_node_slot += 1;

        // Initialize leaf node if termination criteria is met
        if n_primitives <= self.m_max_prim_per_node || depth == 0
        {
            self.m_nodes[node_num].create_leaf(prim_nums, n_primitives, &mut self.m_sorted_indices);
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
        let mut axis = node_bbox.maximum_extent();
        let mut retries = 0;

        loop
        {
            for i in 0..n_primitives
            {
                let _prim_num = prim_nums[i];
                let bbox = all_prim_bbox[_prim_num].clone();

                unsafe
                    {
                        edges_ref[axis][2*i] = BoundEdge::new(bbox.m_vertex_0[axis], _prim_num, true);
                        edges_ref[axis][2*i +1] = BoundEdge::new(bbox.m_vertex_1[axis], _prim_num, false);
                    }
            }

            // Sort the edges.
            edges_ref[axis].sort_by(KDTree::<T>::edge_sort);

            // Compute costs of all splits for axis to find the best cost.
            let mut n_below = 0 as usize;
            let mut n_above = n_primitives;

            for i in 0..(2 * n_primitives)
            {
                if edges_ref[axis][i].m_type == EdgeType::ET_end
                {
                    n_above -= 1;
                }
                let edgeT = edges_ref[axis][i].m_t;
                if edgeT > KDTree::<T>::vector3_index_get(&node_bbox.m_vertex_0, axis) && edgeT < KDTree::<T>::vector3_index_get(&node_bbox.m_vertex_1, axis)
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
            self.m_nodes[node_num].create_leaf(prim_nums, n_primitives, &mut self.m_sorted_indices);
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
        for i in (best_offset + 1)..2*n_primitives
        {
            if edges_ref[best_axis][i].m_type == EdgeType::ET_start
            {
                prims0[n1] = edges_ref[best_axis][i].m_prim_num as u16;
                n1 += 1;
            }
        }

        let t_split = edges_ref[best_axis][best_offset].m_t;
        let mut bbox0 = (*node_bbox).clone();
        let mut bbox1 = (*node_bbox).clone();
        KDTree::<T>::vector3_index_set(&mut bbox0.m_vertex_1, best_axis, t_split);
        KDTree::<T>::vector3_index_set(&mut bbox1.m_vertex_0, best_axis, t_split);

        self.built_tree(node_num + 1, &bbox0, all_prim_bbox,
                        prim_nums, n0, depth - 1, edges_ref,
                        prims0, &mut prims1[n_primitives..], bad_refine);

        let above_child = self.m_next_free_node_slot;
        self.m_nodes[node_num].create_interior(best_axis, above_child, t_split);

        self.built_tree(above_child, &bbox1, all_prim_bbox,
                        prim_nums, n1, depth + 1, edges_ref,
                        prims0, &mut prims1[n_primitives..], bad_refine);

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

    pub fn vector3_index_get(vector: &Vector3<f32>, index: usize) -> f32
    {
        match index
        {
            0 => vector.x,
            1 => vector.y,
            2 => vector.z,
            _ => panic!("Index out of range for Vector3."),
        }
    }

    pub fn vector3_index_set(vector: &mut Vector3<f32>, index: usize, val: f32)
    {
        match index
        {
            0 => { vector.x = val; },
            1 => { vector.y = val; },
            2 => { vector.z = val; },
            _ => panic!("Index out of range for Vector3."),
        }
    }

}

impl<T> Debug for KDTree<T> where T: BoundedConcrete
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("KDTree")
            .finish()
    }
}

impl<T> Geometry for KDTree<T> where T: BoundedConcrete
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        let mut tmin = 0.0_f32;
        let mut tmax = 0.0_f32;
        if !self.m_bounds.calculate_hit_time(incomeray, &mut tmin, &mut tmax)
        {
            return Ok(false);
        }

        let inv_dir = Vector3::new(incomeray.m_direction.x.inv(),
                                   incomeray.m_direction.y.inv(),
                                   incomeray.m_direction.z.inv());

        let mut tasks = vec![KDTasks::default(); 64];
        let mut task_offset = 0;
        let is_hitting = false;
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
                                if self.m_primitives[node_ref.m_pub_union.m_one_primitive as usize].hit(incomeray, time, shaderecord).unwrap_or(false)
                                {
                                    return Ok(true);
                                }
                            }
                        _ =>
                            {
                                let mut time = INFINITY;
                                for i in node_ref.get_primitives_indices_offset()..min(node_ref.get_n_primitives(), self.m_sorted_indices.len())
                                {
                                    let index = self.m_sorted_indices[i];
                                    if self.m_primitives[index].hit(incomeray, &mut time, shaderecord).unwrap_or(false)
                                    {
                                        return Ok(true);
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
                    let t_plane = KDTree::<T>::vector3_index_get(&inv_dir,axis) * (node_ref.get_split_position() - KDTree::<T>::vector3_index_get(&incomeray.m_origin, axis));

                    let mut first_child = null();
                    let mut second_child = null();
                    match KDTree::<T>::vector3_index_get(&incomeray.m_direction, axis) < node_ref.get_split_position()
                        || (KDTree::<T>::vector3_index_get(&incomeray.m_direction, axis) == node_ref.get_split_position()
                        && KDTree::<T>::vector3_index_get(&incomeray.m_direction, axis) < 0.0)
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

        Ok(false)
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
    m_prim_num: usize,
    m_type: EdgeType,
}

impl BoundEdge
{
    pub fn new(t: f32, prim_num: usize, starting: bool) -> BoundEdge
    {
        BoundEdge
        {
            m_t: t,
            m_prim_num: prim_num,
            m_type: if starting { ET_start } else { ET_end },
        }
    }
}

struct KDTreeNode
{
    pub m_pub_union: KDTreeNode_pub_union,
    m_priv_union: KDTreeNode_priv_union,
}

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

    fn create_leaf(&mut self, prim_nums: &mut [usize], np: usize, primitive_indices: &mut Vec<usize>)
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
                        self.m_pub_union.m_prim_indices_offset = primitive_indices.len();
                        primitive_indices.extend_from_slice(&prim_nums[0..np]);
                    }
                }
            }
    }

    fn create_interior(&mut self, axis: usize, ac: usize, s: f32)
    {
        unsafe {
            self.m_pub_union.m_split = s;
            self.m_priv_union.m_flags = axis;
            self.m_priv_union.m_above_child |= ac << 2;
        }
    }

    pub fn get_split_position(&self) -> f32 { unsafe { self.m_pub_union.m_split } }
    pub fn get_n_primitives(&self) -> usize { unsafe { self.m_priv_union.m_n_prims >> 2 } }
    pub fn get_split_axis(&self) -> usize { unsafe { self.m_priv_union.m_flags & 3 } }
    pub fn is_leaf(&self) -> bool { unsafe { self.m_priv_union.m_flags & 3 == 3 } }
    pub fn get_above_child(&self) -> usize { unsafe { self.m_priv_union.m_above_child >> 2 } }
    pub fn get_primitives_indices_offset(&self) -> usize { unsafe { self.m_pub_union.m_prim_indices_offset } }
}

union KDTreeNode_pub_union
{
    m_split: f32,                 // Interior
    m_one_primitive: u8,          // Leaf
    m_prim_indices_offset: usize, // Leaf
}

union KDTreeNode_priv_union
{
    m_flags: usize,
    m_n_prims: usize,
    m_above_child: usize,
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