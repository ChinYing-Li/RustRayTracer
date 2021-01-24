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

pub struct Subregion
{
    m_start_coords: Vector2<usize>,
    m_end_coords:   Vector2<usize>,
    pub m_area: usize
}

impl Subregion
{
    pub fn new(start_coords: (usize, usize), end_coords: (usize, usize)) -> Subregion
    {
        if end_coords.1 < start_coords.1 || end_coords.0 < start_coords.0
        {
            panic!("Invalid pair of coordinates provided")
        }

        Subregion
        {
            m_start_coords: Vector2::new(start_coords.0, start_coords.1),
            m_end_coords: Vector2::new(end_coords.0, end_coords.1),
            m_area: (end_coords.1 - start_coords.1) * (end_coords.0 - start_coords.0),
        }
    }

    pub fn get_start_coords(&self) -> Vector2<usize>
    {
        self.m_start_coords
    }

    pub fn get_end_coords(&self) -> Vector2<usize>
    {
        self.m_end_coords
    }
}