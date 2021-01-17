use cgmath::{Vector2, ElementWise};
use std::sync::atomic;

/// The queue of tasks 
pub struct ComputeQueue
{
    m_dimensions: Vector2<u32>,
    m_block_indices: Vec<Vector2<u32>>,
    m_next: atomic::AtomicUsize
}

pub struct ComputeQueueIter<'a>
{
    m_queue: &'a ComputeQueue,
}

impl ComputeQueue
{
    pub fn new(img_dim: Vector2<u32>, 
               block_dim: Vector2<u32>) -> ComputeQueue
    {
        if img_dim[0] % block_dim[0] || img_dim[1] % block_dim[1]
        {
            panic!("Image dimensions are not multiples for block dimensions");
        }
        
        let n_blocks = img_dim.div_element_wise(block_dim);
        // For now, w assume that we will render all the blocks
        let mut block_indices: Vec<Vector2<u32>> = (0..n_blocks[0]*n_blocks[1])
            .map(|index| Vector2::new(index % n_blocks[0], index / n_blocks[0])).collect();

        ComputeQueue
        {
            m_block_indices: block_indices,
            m_dimensions: block_dim,
            m_next: atomic::AtomicUsize::new(0),
        }
    }

    pub fn get_block_dim(&self) -> Vector2<u32>
    {
        self.m_dimensions
    }

    pub fn len(&self) -> usize
    {
        self.m_block_indices.len()
    }

    pub fn iter(&self) -> ComputeQueueIter
    {
        ComputeQueueIter{ m_queue: self }
    }

    fn next(&self) -> Option<Vector2<u32>>
    {
        let index = self.m_next.fetch_add(1, atomic::Ordering::AcqRel);
        if index >= self.m_block_indices.len() { None }
        else { Some(self.m_block_indices[index]) }
    }
}