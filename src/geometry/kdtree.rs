use std::sync::Arc;
use cgmath::{Vector3, Zero, ElementWise};
use std::cmp::{max, min, Ordering};
use crate::geometry::kdtree::EdgeType::{ET_start, ET_end};
use std::f32::INFINITY;
use std::fmt;
use std::slice;
use cgmath::num_traits::{Inv, AsPrimitive};
use std::borrow::Borrow;
use obj::Obj;
// use std::collections::hash_map::DefaultHasher;

use crate::world::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::geometry::trimesh::TriMesh;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::geometry::{Boundable, Geometry, GeomError, BoundedConcrete, Shadable};
use crate::geometry::bbox::BBox;
use std::ptr::null;

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

    // Constants
    m_empty_bonus: f32,
    m_intersect_cost: f32,
    m_traversal_cost: f32,

    m_num_allocated_nodes: u32,
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
            m_max_depth: if max_depth == 0 { (8.0 + (1.3 * (prim_vec_len as f32).log2()).floor()) as u32 }
                        else { max_depth },
            m_bounds: prim_vec[0].get_bbox(),
            // Need to initialize the bounding box properly
            m_empty_bonus: empty_bonus,
            m_intersect_cost: intersect_cost,
            m_traversal_cost: traversal_cost,

            m_num_allocated_nodes: 0,
            m_next_free_node_slot: 0,
            m_nodes: Vec::with_capacity(0),
            m_material: None,
        }
    }

    pub fn init(&mut self)
    {
        let mut vec_bbox = Vec::with_capacity(self.m_primitives.len());
        print!("max depth {}", self.m_max_depth);
        for primitive in self.m_primitives.iter()
        {
            let b = primitive.get_bbox();
            self.m_bounds = self.m_bounds.union(&b);
            vec_bbox.push(b);
        }

        // Allocate memory for kd-tree construction
        let mut edges_vec = vec![vec![ BoundEdge::default(); 2 * self.m_primitives.len()]; 3];
        // let mut edges = edges_vec.iter_mut().map(|v| v.as_mut_slice()).collect();

        let mut prims0_vec = vec![0_u32; self.m_primitives.len() + 1];
        let prims0_ptr = prims0_vec.as_mut_ptr();

        let mut prims1_vec = vec![0_u32; (self.m_max_depth + 1) as usize * self.m_primitives.len()];
        let prims1_ptr = prims1_vec.as_mut_ptr();

        let mut prim_nums_vec: Vec<u32> = (0..(self.m_primitives.len()+1) as u32).collect();
        let prim_nums_ptr = prim_nums_vec.as_mut_ptr();

        let mut val = 0;
        unsafe
            {
                for i in 0..self.m_primitives.len() as isize
                {
                    val = prim_nums_ptr.offset(i).read();
                    print!("val {}", val);
                }
            }
        // return;

        let bounds = self.m_bounds.clone();
        let n_primitives = self.m_primitives.len();
        let max_depth = self.m_max_depth;

        unsafe {
            KDTree::build_tree_unsafe(self, 0,
                                      &bounds, &vec_bbox,
                                      prim_nums_ptr, n_primitives,
                                      max_depth, &mut edges_vec,
                                      prims0_ptr,
                                      prims1_ptr,
                                      0
            );
        }

        /*
        KDTree::build_tree(self, 0,
                           &bounds, &vec_bbox,
                           &mut prim_nums_vec, n_primitives,
                           max_depth, &mut edges_vec,
                           &mut prims0_vec, 0,
                           &mut prims1_vec, 0,
                           0);
                           */
    }

    /**
     * This method builds the KD-tree recursively.
     */
    fn build_tree(&mut self,
                  node_num: usize,
                  node_bbox: &BBox,
                  all_prim_bbox: &Vec<BBox>,
                  prim_nums: &Vec<u32>,
                  n_primitives: usize,
                  depth: u32,
                  edges_ref: &mut Vec<Vec<BoundEdge>>,
                  prims0: &mut Vec<u32>, prims0_offset: usize,
                  prims1: &mut Vec<u32>, prims1_offset: usize,
                  mut bad_refine: u8)
    {
        assert_eq!(node_num as u32, self.m_next_free_node_slot);

        // Allocate more memory in case we ran out of node slots.
        if self.m_next_free_node_slot == self.m_num_allocated_nodes
        {
            let additional_allocated_nodes = max(2 * self.m_num_allocated_nodes, 512) as usize - self.m_nodes.len();
            let new_len = additional_allocated_nodes + self.m_nodes.len();
            self.m_nodes.resize(new_len, DEFAULT_NODE);
            self.m_num_allocated_nodes = self.m_nodes.len() as u32;
        }

        self.m_next_free_node_slot += 1;

        // Initialize leaf node if termination criteria is met
        if n_primitives as u8 <= self.m_max_prim_per_node || depth == 0
        {
            self.m_nodes[node_num].create_leaf(prim_nums, n_primitives as usize, &mut self.m_sorted_indices);
            return;
        }

        // Else, initialize interior node and continue
        let mut best_axis = 3;
        let mut best_offset = 0;
        let mut best_cost = INFINITY;
        let old_cost = self.m_intersect_cost * n_primitives as f32;
        let inv_total_SA = node_bbox.get_surface_area().inv();
        let diff = node_bbox.get_diagonal();

        // Choose the axis to split
        let mut axis = node_bbox.maximum_extent();
        let mut retries = 0_u8;

        loop
        {
            for i in 0..n_primitives as usize
            {
                let _prim_num = prim_nums[i];
                let bbox = &all_prim_bbox[_prim_num as usize];

                edges_ref[axis][2*i] = BoundEdge::new(bbox.m_vertex_0[axis as usize], _prim_num, true);
                edges_ref[axis][2*i + 1] = BoundEdge::new(bbox.m_vertex_1[axis as usize], _prim_num, false);
            }

            // Sort the edges.
            edges_ref[axis].sort_by(KDTree::<T>::edge_sort);

            // Compute costs of all splits for axis to find the best cost.
            let mut n_below = 0_i32;
            let mut n_above = n_primitives as i32;

            for i in 0..(2 * n_primitives) as usize
            {
                print!("n_above {}\n", n_above);
                if edges_ref[axis][i].m_type == EdgeType::ET_end
                {
                    n_above -= 1;
                }
                let edge_t = edges_ref[axis][i].m_t;
                print!("edge_t {}\n", edge_t);
                print!("vertex 0 {}\n", node_bbox.m_vertex_0[axis]);
                print!("vertex 1 {}\n", node_bbox.m_vertex_1[axis]);

                if edge_t > node_bbox.m_vertex_0[axis]
                    && edge_t < node_bbox.m_vertex_1[axis]
                {
                    // Compute cost for aplit at this edge.
                    // 1. Compute child surface areas.
                    let axis0 = (axis + 1) % 3;
                    let axis1 = (axis + 2) % 3;

                    let below_surfacearea = 2.0 * (diff[axis0] * diff[axis1]
                        + (edge_t - node_bbox.m_vertex_0[axis]) * (diff[axis0] + diff[axis1]));
                    let above_surfacearea = 2.0 * (diff[axis0] * diff[axis1]
                        + (node_bbox.m_vertex_1[axis] - edge_t) * (diff[axis0] + diff[axis1]));

                    let percentage_below = below_surfacearea * inv_total_SA;
                    let percentage_above = above_surfacearea * inv_total_SA;

                    let empty_bonus = if n_above == 0 || n_below == 0 { self.m_empty_bonus } else { 0.0 };
                    let cost = self.m_traversal_cost +
                        self.m_intersect_cost * (1.0 - empty_bonus) * (n_below as f32 * percentage_below + n_above as f32 * percentage_above);

                    print!("cost {}\n", cost);
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
            print!("best axis {}, retries {}\n", best_axis, retries);
            if best_axis == 3 && retries < 3
            {
                axis = (axis + 1) % 3;
                retries += 1;
            }
            else
            { break; }
        }

        bad_refine += if best_cost > old_cost { 1 } else { 0 };
        if (best_cost > 4.0 * old_cost && n_primitives < 16) || best_axis == 3 || bad_refine == 3
        {
            self.m_nodes[node_num].create_leaf(prim_nums,
                                               n_primitives as usize,
                                               &mut self.m_sorted_indices);
            return;
        }

        // Classify primitives wrt the split
        let mut n0 = 0;
        let mut n1 = 0;
        for i in 0..best_offset
        {
            if edges_ref[best_axis][i].m_type == EdgeType::ET_start
            {
                prims0[prims0_offset + n0] = edges_ref[best_axis][i].m_prim_num;
                n0 += 1;
            }
        }

        for i in (best_offset + 1)..2 * n_primitives as usize
        {
            if edges_ref[best_axis][i].m_type == EdgeType::ET_start
            {
                prims1[prims1_offset + n1] = edges_ref[best_axis][i].m_prim_num;
                n1 += 1;
            }
        }

        let t_split = edges_ref[best_axis][best_offset].m_t;
        let mut bbox0 = (*node_bbox).clone();
        let mut bbox1 = (*node_bbox).clone();
        bbox0.m_vertex_1[best_axis] = t_split;
        bbox1.m_vertex_0[best_axis] = t_split;
        /*
        self.build_tree(node_num + 1, &bbox0, all_prim_bbox,
                        prims0, n0, depth - 1, edges_ref,
                        prims0, prims0_offset,
                        prims1, prims1_offset + n_primitives,
                        bad_refine);

        let above_child = self.m_next_free_node_slot as usize;
        self.m_nodes[node_num].create_interior(best_axis as u8, above_child as u32, t_split);

        self.build_tree(above_child, &bbox1, all_prim_bbox,
                        prims1, n1, depth - 1, edges_ref,
                        prims0, prims0_offset,
                        prims1, prims1_offset + n_primitives,
                        bad_refine);
        */
    }

    unsafe fn build_tree_unsafe(&mut self,
                  node_num: usize,
                  node_bbox: &BBox,
                  all_prim_bbox: &Vec<BBox>,
                  prim_nums: *mut u32,
                  n_primitives: usize,
                  depth: u32,
                  edges_ref: &mut Vec<Vec<BoundEdge>>,
                  prims0: *mut u32,
                  prims1: *mut u32,
                  mut bad_refine: u8)
    {
        assert_eq!(node_num as u32, self.m_next_free_node_slot);
        print!("depth {}", depth);

        // Allocate more memory in case we ran out of node slots.
        if self.m_next_free_node_slot == self.m_num_allocated_nodes
        {
            let additional_allocated_nodes = max(2 * self.m_num_allocated_nodes, 512) as usize - self.m_nodes.len();
            let new_len = additional_allocated_nodes + self.m_nodes.len();
            self.m_nodes.resize(new_len, DEFAULT_NODE);
            self.m_num_allocated_nodes = self.m_nodes.len() as u32;
        }

        self.m_next_free_node_slot += 1;

        // Initialize leaf node if termination criteria is met
        if n_primitives as u8 <= self.m_max_prim_per_node || depth == 0
        {
            self.m_nodes[node_num].create_leaf_unsafe(prim_nums, n_primitives as usize, &mut self.m_sorted_indices);
            return;
        }

        // Else, initialize interior node and continue
        let mut best_axis = 3;
        let mut best_offset = 0;
        let mut best_cost = INFINITY;
        let old_cost = self.m_intersect_cost * n_primitives as f32;
        let inv_total_SA = node_bbox.get_surface_area().inv();
        let diff = node_bbox.get_diagonal();

        // Choose the axis to split
        let mut axis = node_bbox.maximum_extent();
        let mut retries = 0_u8;

        loop
        {
            for i in 0..n_primitives as usize
            {
                let _prim_num = prim_nums.offset(i as isize).read();
                // print!("prim_num value {} \n", _prim_num);
                let bbox = &all_prim_bbox[_prim_num as usize];

                edges_ref[axis][2 * i] = BoundEdge::new(bbox.m_vertex_0[axis], _prim_num, true);
                edges_ref[axis][2 * i + 1] = BoundEdge::new(bbox.m_vertex_1[axis], _prim_num, false);
            }

            // Sort the edges.
            edges_ref[axis].sort_by(KDTree::<T>::edge_sort);

            // Compute costs of all splits for axis to find the best cost.
            let mut n_below = 0_i32;
            let mut n_above = n_primitives as i32;

            for i in 0..(2 * n_primitives) as usize
            {
                print!("n_above {}\n", n_above);
                if edges_ref[axis][i].m_type == EdgeType::ET_end
                {
                    n_above -= 1;
                }

                let edge_t = edges_ref[axis][i].m_t;
                print!("edge_t {}\n", edge_t);
                print!("vertex 0 {}\n", node_bbox.m_vertex_0[axis]);
                print!("vertex 1 {}\n", node_bbox.m_vertex_1[axis]);

                if edge_t > node_bbox.m_vertex_0[axis]
                    && edge_t < node_bbox.m_vertex_1[axis]
                {
                    // Compute cost for aplit at this edge.
                    // 1. Compute child surface areas.
                    let axis0 = (axis + 1) % 3;
                    let axis1 = (axis + 2) % 3;

                    let below_surfacearea = 2.0 * (diff[axis0] * diff[axis1]
                        + (edge_t - node_bbox.m_vertex_0[axis]) * (diff[axis0] + diff[axis1]));
                    let above_surfacearea = 2.0 * (diff[axis0] * diff[axis1]
                        + (node_bbox.m_vertex_1[axis] - edge_t) * (diff[axis0] + diff[axis1]));

                    let percentage_below = below_surfacearea * inv_total_SA;
                    let percentage_above = above_surfacearea * inv_total_SA;

                    let empty_bonus = if n_above == 0 || n_below == 0 { self.m_empty_bonus } else { 0.0 };
                    let cost = self.m_traversal_cost +
                        self.m_intersect_cost * (1.0 - empty_bonus) * (n_below as f32 * percentage_below + n_above as f32 * percentage_above);

                    print!("cost {}\n", cost);
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
            print!("best axis {}, retries {}\n", best_axis, retries);
            if best_axis == 3 && retries < 3
            {
                axis = (axis + 1) % 3;
                retries += 1;
            }
            else { break; }
        }

        bad_refine += if best_cost > old_cost { 1 } else { 0 };
        if (best_cost > 4.0 * old_cost && n_primitives < 16) || best_axis == 3 || bad_refine == 3
        {
            self.m_nodes[node_num].create_leaf_unsafe(prim_nums,
                                               n_primitives as usize,
                                               &mut self.m_sorted_indices);
            return;
        }

        // Classify primitives wrt the split
        let mut n0 = 0;
        let mut n1 = 0;
        for i in 0..best_offset
        {
            if edges_ref[best_axis][i].m_type == EdgeType::ET_start
            {
                let mut prims0_ptr = prims0.offset(n0);
                prims0_ptr.write(edges_ref[best_axis][i].m_prim_num);
                n0 += 1;
            }
        }

        for i in (best_offset + 1)..2 * n_primitives as usize
        {
            if edges_ref[best_axis][i].m_type == EdgeType::ET_start
            {
                let mut prims1_ptr = prims0.offset(n1);
                prims1_ptr.write(edges_ref[best_axis][i].m_prim_num);
                n1 += 1;
            }
        }

        let t_split = edges_ref[best_axis][best_offset].m_t;
        let mut bbox0 = (*node_bbox).clone();
        let mut bbox1 = (*node_bbox).clone();
        bbox0.m_vertex_1[best_axis] = t_split;
        bbox1.m_vertex_0[best_axis] = t_split;

        self.build_tree_unsafe(node_num + 1, &bbox0, all_prim_bbox,
                               prims0, n0 as usize, depth - 1, edges_ref,
                               prims0,
                               prims1.offset(n_primitives as isize),
                               bad_refine);

        let above_child = self.m_next_free_node_slot as usize;
        self.m_nodes[node_num].create_interior(best_axis as u8, above_child as u32, t_split);

        self.build_tree_unsafe(above_child, &bbox1, all_prim_bbox,
                               prims1, n1 as usize, depth - 1, edges_ref,
                               prims0,
                               prims1.offset(n_primitives as isize),
                               bad_refine);

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

        let mut hit: (bool, u32) = (false, 0);
        let mut current_offset = 0_usize;
        let mut first_child_offset = 0_usize;
        let mut second_child_offset = 0_usize;

        while current_offset < self.m_nodes.len() && task_offset < 64
        {
            if self.m_nodes[current_offset].is_leaf()
            {
                let n_primitives = self.m_nodes[current_offset].get_n_primitives();
                match n_primitives
                {
                    1 =>
                        unsafe {
                            let index = self.m_nodes[current_offset].m_pub_union.m_one_primitive;
                            if self.m_primitives[index as usize].shadow_hit(shadowray, time)
                            {
                                hit = (true, index as u32);
                            }
                        }
                    _ =>
                        {
                            let mut time_temp = INFINITY;
                            for i in 0..n_primitives as usize
                            {
                                let index = self.m_sorted_indices[i +
                                    self.m_nodes[current_offset].get_primitives_indices_offset() as usize];
                                if self.m_primitives[index as usize].shadow_hit(shadowray, &mut time_temp)
                                {
                                    hit = (true, index);
                                }
                            }
                        }
                }

                if task_offset > 0
                {
                    task_offset -= 1;
                    current_offset = tasks[task_offset].m_node_offset;
                    tmin = tasks[task_offset].m_tmin;
                    tmax = tasks[task_offset].m_tmax;
                } else { break; }
            }
            else
            {
                let split_axis = self.m_nodes[current_offset].get_split_axis() as usize;
                let split_position = self.m_nodes[current_offset].get_split_position();
                let t_plane = inv_dir[split_axis] * (split_position - shadowray.m_origin[split_axis]);

                match shadowray.m_direction[split_axis] < split_position
                    || (shadowray.m_direction[split_axis] == split_position && shadowray.m_direction[split_axis] <= 0.0)
                {
                    true =>
                        unsafe {
                            first_child_offset = current_offset + 1;
                            second_child_offset = self.m_nodes[current_offset].m_priv_union.m_above_child as usize;
                        }
                    false =>
                        unsafe {
                            first_child_offset = self.m_nodes[current_offset].m_priv_union.m_above_child as usize;
                            second_child_offset = current_offset + 1;
                        }
                }
                // Advance to next child node, possibly enqueue another children
                if t_plane > tmax || t_plane <= 0.0 { current_offset = first_child_offset; }
                else if t_plane < tmin { current_offset = second_child_offset; }
                else
                {
                    tasks[task_offset].m_node_offset = second_child_offset;
                    tasks[task_offset].m_tmin = t_plane;
                    tasks[task_offset].m_tmax = tmax;
                    task_offset += 1;
                    current_offset = first_child_offset;
                    tmax = t_plane;
                }
            }
        }
        hit
    }
}

impl<T> fmt::Debug for KDTree<T> where T: BoundedConcrete + Clone
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        let mut dummy_sr = ShadeRec::get_dummy();
        match self.custom_shadow_hit(shadowray, tmin, &mut dummy_sr)
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

    fn create_leaf(&mut self, prim_nums: &Vec<u32>, np: usize, primitive_indices: &mut Vec<u32>)
    {
        unsafe // Modify unions
            {
                print!("np {}", np);
                self.m_priv_union.m_flags = 3;
                print!("prim_num_vec {}\n", prim_nums.len());
                self.m_priv_union.m_num_prims_overlapped |= (np as u32) << 2;

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

    fn create_leaf_unsafe(&mut self, prim_nums: *mut u32, np: usize, primitive_indices: &mut Vec<u32>)
    {
        unsafe // Modify unions
            {
                print!("np {}", np);
                self.m_priv_union.m_flags = 3;
                self.m_priv_union.m_num_prims_overlapped |= (np as u32) << 2;

                match np
                {
                    0 => { self.m_pub_union.m_one_primitive = 0; }
                    1 => { self.m_pub_union.m_one_primitive = prim_nums.read() as u8; }
                    _ => {
                        self.m_pub_union.m_prim_indices_offset = primitive_indices.len() as u32;
                        primitive_indices.extend_from_slice(slice::from_raw_parts(prim_nums, np));
                    }
                }
            }
    }

    fn create_interior(&mut self, axis: u8, child_above: u32, s: f32)
    {
        unsafe {
            self.m_pub_union.m_split = s;
            self.m_priv_union.m_flags = axis;
            self.m_priv_union.m_above_child |= child_above << 2;
        }
    }

    pub fn get_split_position(&self) -> f32 { unsafe { self.m_pub_union.m_split } }
    pub fn get_n_primitives(&self) -> u32 { unsafe { self.m_priv_union.m_num_prims_overlapped >> 2 } }
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
    m_num_prims_overlapped: u32,
    m_above_child: u32,
}

#[derive(Clone, Debug)]
struct KDTasks
{
    m_node_offset: usize,
    m_tmin: f32,
    m_tmax: f32,
}

impl KDTasks
{
    pub fn new() -> KDTasks
    {
        KDTasks
        {
            m_node_offset: 0,
            m_tmin: INFINITY,
            m_tmax: 0.0,
        }
    }

    pub fn is_initialized(&self) -> bool
    {
        unimplemented!()
    }
}

impl Default for KDTasks
{
    fn default() -> Self {
        KDTasks::new()
    }
}