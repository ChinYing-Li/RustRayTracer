use cgmath::{Vector3, InnerSpace, dot, ElementWise, Zero, Vector2};
use crate::utils::color::Colorf;
use std::sync::Mutex;
use image::error::UnsupportedErrorKind::Color;
use crate::utils::colorconstant::COLOR_BLACK;

/// The buffer that stores our render result while we are rendering
pub struct RenderBuffer
{
    m_dim: Vector2<usize>,
    m_block_dim: Vector2<usize>, // The Render buffer is divided into blocks for multithreading
    m_samples: Vec<Mutex<Vec<Colorf>>>,
}

impl RenderBuffer
{
    pub fn new(width: usize, height: usize, block_width: usize, block_height: usize) -> RenderBuffer
    {
        let num_blocks = (width / block_width, height / block_height);
        let samples = vec![Mutex::new(vec![COLOR_BLACK; block_width * block_height]);
                           (num_blocks.0 * num_blocks.1) as usize];
        RenderBuffer
        {
            m_dim: Vector2::new(width, height),
            m_block_dim: Vector2::new(block_width, block_height),
            m_samples: samples,
        }
    }

    pub fn write(&mut self)
    {

    }

    pub fn clear(&mut self)
    {

    }

    pub fn get_dim(&self) -> Vector2<usize>
    {
        self.m_dim
    }
}
