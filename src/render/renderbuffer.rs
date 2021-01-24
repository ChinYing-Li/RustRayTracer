use cgmath::{Vector3, InnerSpace, dot, ElementWise, Zero, Vector2};
use std::sync::Mutex;
use image::error::UnsupportedErrorKind::Color;
use std::iter;

use crate::utils::color::Colorf;
use crate::utils::colorconstant::COLOR_BLACK;

/// The buffer that stores our render result while we are rendering
pub struct RenderBuffer
{
    m_dim: Vector2<usize>,
    m_block_dim: Vector2<usize>, // The Render buffer is divided into blocks for multithreading
    m_num_blocks: Vector2<usize>,
    m_samples: Vec<Mutex<Vec<Colorf>>>,
}

impl RenderBuffer
{
    pub fn new(width: usize, height: usize, block_width: usize, block_height: usize) -> RenderBuffer
    {
        let num_blocks = Vector2::new(width / block_width, height / block_height);
        let mut samples = Vec::with_capacity(num_blocks.x * num_blocks.y);
        for _ in 0..num_blocks.x * num_blocks.y
        {
            samples.push(Mutex::new(vec![COLOR_BLACK; block_width * block_height]));
        }

        RenderBuffer
        {
            m_dim: Vector2::new(width, height),
            m_block_dim: Vector2::new(block_width, block_height),
            m_num_blocks: num_blocks,
            m_samples: samples,
        }
    }

   /// Write the image samples to a certain block
    pub fn write(&mut self, samples: &Vec<Colorf>, block_nx: usize, block_ny: usize)
    {
        // TODO: this simply overwrites the block; could try something fancier
        assert!(block_nx < self.m_num_blocks.x && block_ny < self.m_num_blocks.y);
        let block_index = block_nx * self.m_num_blocks.x + block_ny;
        for (pixels, color) in self.m_samples[block_index].lock().unwrap()
            .iter_mut().zip(samples.iter())
        {
            *pixels = *color;
        }
    }

    pub fn clear(&mut self)
    {
        for locked_block in self.m_samples.iter()
        {
            let mut block = locked_block.lock().unwrap();
            block.iter_mut().map(|pixel| *pixel = COLOR_BLACK).count();
        }
    }

    pub fn get_dim(&self) -> Vector2<usize>
    {
        self.m_dim
    }
}
