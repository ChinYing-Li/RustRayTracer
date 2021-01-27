use cgmath::{Vector3, InnerSpace, dot, ElementWise, Zero, Vector2};
use std::sync::{Mutex, atomic};
use std::iter;

use crate::utils::color::Colorf;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::render::renderdata::RenderMeta;
use std::option::Iter;
use crate::output::OutputManager;

/// The buffer that stores our render result while we are rendering
pub struct RenderBuffer
{
    m_dim: (usize, usize),
    m_block_dim: (usize, usize), // The Render buffer is divided into blocks for multithreading
    m_num_blocks: (usize, usize),
    m_sample_blocks: Vec<(RenderMeta, Mutex<Vec<Colorf>>)>,
    m_next: atomic::AtomicUsize,
}

impl RenderBuffer
{
    pub fn new(img_dim: (usize, usize),
               block_dim: (usize, usize)) -> RenderBuffer
    {
        if img_dim.0 % block_dim.0 != 0 || img_dim.1 % block_dim.1 != 0
        {
            panic!("Image dimensions are not multiples for block dimensions");
        }
        let num_blocks = (img_dim.0 / block_dim.0, img_dim.1 / block_dim.1);
        let mut samples = Vec::with_capacity(num_blocks.0 * num_blocks.1);

        for i in 0..num_blocks.0 - 1
        {
            for j in 0..num_blocks.1 - 1
            {
                samples.push((RenderMeta::new((i * block_dim.0, j * block_dim.1),
                                              ((i+1) * block_dim.0, (j+1) * block_dim.1),
                                              (i, j)),
                              Mutex::new(vec![COLOR_BLACK; block_dim.0 * block_dim.1])));
            }
        }

        RenderBuffer
        {
            m_dim: img_dim,
            m_block_dim: block_dim,
            m_num_blocks: num_blocks,
            m_sample_blocks: samples,
            m_next: atomic::AtomicUsize::new(0),
        }
    }

   /// Read an array of pixels into a block
    pub fn read(&self, samples: Vec<Colorf>, rendermeta: &RenderMeta)
   {
       let block_indices = rendermeta.get_block_indices();
        assert!(block_indices.0 < self.m_num_blocks.0 && block_indices.1 < self.m_num_blocks.1);
        let index = block_indices.0 + block_indices.1 * self.m_num_blocks.0;

        for (pixels, color) in self.m_sample_blocks[index].1.lock().unwrap()
            .iter_mut().zip(samples.iter())
        {
            *pixels = *color;
        }
    }

    /// Write all blocks to the OutputManager
    pub fn write(&self, out_manager: &mut dyn OutputManager)
    {
        for (meta, locked_block) in self.m_sample_blocks.iter()
        {
            let start_coord = meta.get_start_coords().max(&(0_usize, 0_usize));
            let img_dim = out_manager.get_img_dim();
            let end_coord = meta.get_end_coords().min(&img_dim);
            let block = locked_block.lock().unwrap();

            for i in start_coord.0..end_coord.0
            {
                for j in start_coord.1..end_coord.1
                {
                    out_manager.write_pixel(i, j, block[i + j * self.m_block_dim.0], 1.0);
                }
            }
        }
    }

    pub fn clear(&mut self)
    {
        for (_, locked_block) in self.m_sample_blocks.iter()
        {
            let mut block = locked_block.lock().unwrap();
            block.iter_mut().map(|pixel| *pixel = COLOR_BLACK).count();
        }
    }

    pub fn get_dim(&self) -> (usize, usize)
    {
        self.m_dim
    }

    pub fn get_block_dim(&self) -> (usize, usize)
    {
        self.m_block_dim
    }

    pub fn iter(&self) -> RenderBufferIter
    {
        RenderBufferIter
        {
            m_queue: self.m_sample_blocks.iter()
                .map(|block| block.0).collect(),
            m_buffer: self
        }
    }

}

pub struct RenderBufferIter<'a>
{
    pub m_queue: Vec<RenderMeta>,
    m_buffer: &'a RenderBuffer,
}

impl RenderBufferIter<'_>
{
    fn next(&self) -> Option<RenderMeta>
    {
        let index = self.m_buffer.m_next.fetch_add(1, atomic::Ordering::AcqRel);
        if index >= self.m_queue.len() { None }
        else { Some(self.m_queue[index]) }
    }
}

impl Iterator for RenderBufferIter<'_>
{
    type Item = RenderMeta;
    fn next(&mut self) -> Option<RenderMeta>
    {
        let index = self.m_buffer.m_next.fetch_add(1, atomic::Ordering::AcqRel);
        if index >= self.m_queue.len() { None }
        else { Some(self.m_queue[index]) }
    }
}