use crate::geometry::Boundable;
use crate::geometry::bbox::BBox;
use std::sync::Arc;
use cgmath::{Vector3, Zero};
use std::cmp::max;
//use crate::geometry::kdtree::EdgeType::{ET_start, ET_end};
/*
pub struct KDTree<T> where T: Boundable
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
    pub fn new(prim_vec: Vec<Arc<dyn T>>,
                intersect_cost: i16,
                traversal_cost: i16,
                empty_bonus: float,
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
        let mut prim_nums = Box::<[BoundEdge]>::new();

    }

    fn built_tree()
    {

    }
}

enum EdgeType
{
    ET_start,
    ET_end,
}

struct BoundEdge
{
    m_t: f32,
    m_prim_num: u16,
    m_type: EdgeType,
}

impl BoundEdge
{
    pub fn new(t: f32, prim_num: u16, starting: bool) -> BoundEdge
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
    pub fn create_leaf(&mut self, np: u16, primitive_indices: &Vec<u16>)
    {
        self.m_priv_union.m_flags = 3;
        self.m_priv_union.m_n_prims |= np << 2;

        match np
        {
            0 => { self.m_pub_union.m_one_primitive = 0; }
            1 => { self.m_pub_union.m_one_primitive = prim_nums[0] }
            _ => {
                self.m_pub_union.m_prim_indices_offset = primitive_indices.len();
            }
        }
    }

    pub fn create_interior(&self);

    pub fn get_split_position(&self) -> f32 { self.m_pub_union.m_split }
    pub fn get_n_primitives(&self) -> u16 { self.m_priv_union.m_n_prims >> 2 }
    pub fn get_split_axis(&self) -> u8 { self.m_priv_union.m_flags & 3 }
    pub fn is_leaf(&self) -> bool { self.m_priv_union.m_flags & 3 == 3 }
    pub fn get_above_child(&self) -> u16 { self.m_priv_union.m_above_child >> 2 }

}

union KDTreeNode_pub_union
{
    m_split: f32,                // Interior
m_one_primitive: u8,         // Leaf
m_prim_indices_offset: usize,   // Leaf
}

union KDTreeNode_priv_union
{
    m_flags: u8,
    m_n_prims: u16,
    m_above_child: u16,
}
*/