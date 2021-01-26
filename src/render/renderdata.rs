use cgmath::Vector2;

use crate::utils::color::Colorf;

pub struct RenderSample
{
    m_pix_coord: Vector2<u32>,
    m_color: Colorf
}

impl RenderSample
{
    pub fn new(pix_x: u32, pix_y: u32, color: Colorf) -> RenderSample
    {
        RenderSample
        {
            m_pix_coord: Vector2::new(pix_x, pix_y),
            m_color: color,
        }
    }
}

/// MetaData for RenderBuffer's sample blocks
#[derive(Clone, Copy)]
pub struct RenderMeta
{
    m_start_coords: (usize, usize),
    m_end_coords:   (usize, usize),
    m_block_index: (usize, usize),
    pub m_area: usize
}

impl RenderMeta
{
    pub fn new(start_coords: (usize, usize),
               end_coords: (usize, usize),
                block_index: (usize, usize)) -> RenderMeta
    {
        if end_coords.1 < start_coords.1 || end_coords.0 < start_coords.0
        {
            panic!("Invalid pair of coordinates provided")
        }

        RenderMeta
        {
            m_start_coords: start_coords,
            m_end_coords: end_coords,
            m_block_index: block_index,
            m_area: (end_coords.1 - start_coords.1) * (end_coords.0 - start_coords.0),
        }
    }

    pub fn get_start_coords(&self) -> &(usize, usize)
    {
        &self.m_start_coords
    }

    pub fn get_end_coords(&self) -> &(usize, usize)
    {
        &self.m_end_coords
    }

    pub fn get_block_indices(&self) -> &(usize, usize)
    {
        &self.m_block_index
    }
}