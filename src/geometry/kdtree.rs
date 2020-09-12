/*use crate::geometry::{Boundable, Geometry, GeomError};
use crate::geometry::bbox::BBox;
use std::sync::Arc;
use cgmath::{Vector3, Zero};
use std::cmp::max;
use crate::geometry::kdtree::EdgeType::{ET_start, ET_end};
use arrayvec::ArrayVec;
use std::f32::INFINITY;
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;
use std::fmt::{Debug, Formatter};
use std::fmt;
use cgmath::num_traits::Inv;
use std::ptr::null;

pub struct KDTree<T: ?Sized> where T: Boundable
{
    pub m_primitives: Vec<Arc<dyn T>>,
    pub m_sorted_indices: Vec<usize>,

    pub m_max_prim_per_node: usize,
    pub m_max_depth: u16,
    pub m_bounds: BBox,

    // Constants      ///////////
    m_empty_bonus: f32,
    m_intersect_cost: i16,
    m_traversal_cost: i16,

    m_number_of_allocated_nodes: usize,
    m_next_free_node_slot: usize,
    m_nodes: Vec<KDTreeNode>,
}

impl<T> KDTree<T> where T: Boundable
{
    const MAX_KDTREE_TASKS: u8 = 64;

    pub fn new(prim_vec: Vec<Arc<dyn T>>,
                intersect_cost: i16,
                traversal_cost: i16,
                empty_bonus: f32,
                max_prim_per_node: usize,
                max_depth: u16) -> KDTree<T>
    {
        KDTree
        {
            m_primitives: prim_vec,
            m_sorted_indices: Vec::with_capacity(prim_vec.len()),

            m_max_prim_per_node: max_prim_per_node,
            m_max_depth: if max_depth <= 0 { 8 + (1.3 * prim_vec.len() as f32).floor() }
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
            let b = primitive.get_bounding_box();
            self.m_bounds = self.m_bounds.union(b);
            vec_bbox.push(b);
        }

        // Allocate memory for kd-tree construction
        let mut edges = vec![vec![ BoundEdge::default(); 2 * self.m_primitives.len()]; 3].into_boxed_slice();
        let mut prims0 = vec![0_i16; self.m_primitives.len()].into_boxed_slice();
        let mut prims1 = vec![0; (self.m_max_depth + 1) * self.m_primitives.len()].into_boxed_slice();

        // Boxing prim_nums
        let arr_prim_nums: Vec<usize> = (0..self.m_primitives.len()).collect();
        let mut prim_nums = arr_prim_nums.into_boxed_slice();

        self.built_tree(0, &self.m_bounds, &vec_bbox,
                        prim_nums, self.m_primitives.len(), self.m_max_depth,
                        edges, prims0, prims1, 0);
    }

    fn built_tree(&mut self, node_num: usize, node_bbox: &BBox, all_prim_bbox: &Vec<BBox>,
                  prim_nums: Box<[usize]>, n_primitives: usize, depth: u16,
                  mut edges: Box<[Vec<BoundEdge>]>, prims0: Box<[i16]>, prims1: Box<[i16]>
                  , mut bad_refine: u8)
    {
        assert_eq!(node_num as usize, self.m_next_free_node_slot);

        // Allocate more memory in case we ran out of node slots.
        if self.m_next_free_node_slot == self.m_number_of_allocated_nodes
        {
            let new_number_of_allocated_nodes = max(2 * self.m_number_of_allocated_nodes, 512);
            self.m_nodes.reserve(new_number_of_allocated_nodes);
            self.m_number_of_allocated_nodes = new_number_of_allocated_nodes;
        }
        self.m_next_free_node_slot += 1;

        // Initialize leaf node if termination criteria met
        if n_primitives <= *self.m_max_prim_per_node || depth == 0
        {
            self.m_nodes[node_num];
            return;
        }

        // Else, initialize interior node and continue
        let mut best_axis = -1;
        let mut best_offset = -1;
        let mut best_cost = INFINITY;
        let old_cost = self.m_intersect_cost * n_primitives as f32;
        let total_surfacearea = node_bbox.get_surface_area();
        let diff = node_bbox.get_diagonal();

        // Choose the axis to split
        let mut axis = node_bbox.maximum_extent();
        let mut retries = 0;

        // a goto statement here!

        for i in 0..n_primitives
        {
            let _prim_num = prim_nums[i];
            let bbox = &all_prim_bbox[_prim_num];
            edges[axis][2*i] = BoundEdge::new(bbox.m_vertex_0[axis], _prim_num, false);
            edges[axis][2*i +1] = BoundEdge::new(bbox.m_vertex_1[axis], _prim_num, false);
        }

        // Sort the edges.
        edges.sort_by(KDTree::edge_sort);

        // Compute costs of all splits for axis to find the best cost.
        let mut n_below = 0 as usize;
        let mut n_above = *n_primitives;

        for i in 0..(2*n_primitives)
        {
            if edges[axis][i].m_type == EdgeType::ET_end
            {
                n_above -= 1;
            }
             let edge_T = edges[axis][i].m_t;
            if edge_T > node_bbox.m_vertex_0[axis] && edge_T < node_bbox.m_vertex_1[axis]
            {
                // Compute cost for aplit at this edge.
                // 1. Compute child surface areas.


                let percentage_below = below_surfacearea * total_surfacearea ;
                let percentage_above = above_surfacearea * total_surfacearea;

                let cost = self.m_traversal_cost +
                    self.m_intersect_cost * (1 - if true{0} else {0.0}) * (n_below * percentage_below + n_above * percentage_above);

                if cost < best_cost
                {
                    best_axis = axis;
                    best_cost = cost;
                    best_offset = i;
                }
            }
            if edges[axis][i].m_type == EdgeType::ET_start
            {
                n_below += 1;
            }
        }

        // If we can't find a good split on this axis, try another axis.
        if best_axis == -1 && retries < 2
        {
            axis = (axis + 1) % 3;
            retries += 1;

            // TODO a goto statement in c++ code!!!
        }

        bad_refine += if best_cost > old_cost { 1 } else { 0 };
        if (best_cost > 4 * old_cost && n_primitives < 16) || best_axis == -1 || bad_refine == 3
        {
            self.m_nodes[node_num].create_leaf(&prim_nums, n_primitives /*primitive indices*/);
        }

        // Classify primitives wrt the split.else
        let mut n0 = 0;
        let mut n1 = 0;
        for i in 0..best_offset
        {
            if edges[best_axis][i].m_type == EdgeType::ET_start
            {
                prims0[n0] = edges[best_axis][i].m_prim_num as i16;
                n0 += 1;
            }
        }
        for i in (best_offset+1)..2*n_primitives
        {
            if edges[best_axis][i].m_type == EdgeType::ET_start
            {
                prims0[n1] = edges[best_axis][i].m_prim_num as i16;
                n1 += 1;
            }
        }

        let t_split = edges[best_axis][best_offset].m_t;
        let mut bbox0 = (*node_bbox).clone();
        let mut bbox1 = (*node_bbox).clone();
        bbox0.m_vertex_1[best_axis] = t_split;
        bbox1.m_vertex_0[best_axis] = t_split;

        self.built_tree(node_num + 1, &bbox0, all_prim_bbox,
                        prims0, n0, depth - 1, edges, prims0, prims1);
        let above_child = self.m_next_free_node_slot;
        self.m_nodes[node_num].create_interior(best_axis, above_child, t_split);

        self.built_tree(above_child, &bbox1, all_prim_bbox,
                        prims1, n1, depth + 1, edges, prims0, prims1);

    }

    fn edge_sort(lhs: &BoundEdge, rhs: &BoundEdge) -> bool
    {
        match lhs.m_t == rhs.m_t
        {
            true => (lhs.m_type)  < (rhs.m_type ),
            _ => lhs.m_t < rhs.m_t
        }
    }
}

impl<T> Geometry for KDTree<T> where T: Boundable
{

    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        if !self.m_bounds.hit(incomeray, time, shaderecord).unwrap()
        {
            return Ok(false);
        }

        let inv_dir = Vector3::new(incomeray.m_direction.x.inv(),
                                                incomeray.m_direction.y.inv(),
                                                incomeray.m_direction.z.inv());

        let is_hitting = false;
        let node_ptr = &self.m_nodes[0];

        while node_ptr != null()
        {
            let axis = *node_ptr.get_split_axis();
            let t_plane = (*node_ptr.get_split_position() - incomeray.m_origin)
                .mul_element_wise(inv_dir);

            let mut first_child = null();
            let mut second_child = null();
            match incomeray.m_direction[axis] < *node_ptr.get_split_position()
                                    || (incomeray.m_direction[axis] == *node_ptr && incomeray.m_direction[axis] < 0.0)
            {
                true =>
                    {
                        first_child = node_ptr + 1;
                        second_child = &self.m_nodes[*node_ptr.m_priv_union.m_above_child];
                    }
                false =>
                    {
                        first_child = &self.m_nodes[*node_ptr.m_priv_union.m_above_child];
                        second_child = node_ptr + 1;
                    }
            }

            // Advance to next child node, possibly enqueue another child


        }
    }
}

impl<T> Debug for KDTree<T> where T: Boundable
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("KDTree")
            .finish()
    }
}

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

#[derive(Debug, Default)]
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
            m_pub_union: KDTreeNode_pub_union{ },
            m_priv_union: KDTreeNode_priv_union{ },
        }
    }

    fn create_leaf(&mut self, np: &u16, primitive_indices: &Vec<u16>)
    {
        unsafe // Modify unions
            {
                self.m_priv_union.m_flags = 3;
                self.m_priv_union.m_n_prims |= *np << 2;

                match *np
                {
                    0 => { self.m_pub_union.m_one_primitive = 0; }
                    1 => { self.m_pub_union.m_one_primitive = 1; }
                    _ => {
                        self.m_pub_union.m_prim_indices_offset = primitive_indices.len();
                    }
                }
            }
    }

    fn create_interior(&mut self, axis: usize, ac: u8, s: f32)
    {
        self.m_pub_union.m_split = s;
        self.m_priv_union.m_flags = axis;
        self.m_priv_union.m_above_child |= ac << 2;
    }

    pub fn get_split_position(&self) -> f32 { self.m_pub_union.m_split }
    pub fn get_n_primitives(&self) -> u16 { self.m_priv_union.m_n_prims >> 2 }
    pub fn get_split_axis(&self) -> u8 { self.m_priv_union.m_flags & 3 }
    pub fn is_leaf(&self) -> bool { self.m_priv_union.m_flags & 3 == 3 }
    pub fn get_above_child(&self) -> u16 { self.m_priv_union.m_above_child >> 2 }

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
    m_n_prims: u16,
    m_above_child: usize,
}

struct KDTasks
{
    m_tmin: f32,
    m_tmax: f32,
}
*/