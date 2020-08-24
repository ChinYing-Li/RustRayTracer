use std::{f32};
use cgmath::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct ViewPlane
{
    pub m_hres: u16,
    pub m_vres: u16,
    pub m_pixsize: f32,
    m_gamma: f32,
    m_invgamma: f32,

    pub m_maxdepth: u16,
    pub m_numsample: u16,
}

impl ViewPlane
{
    /// Default constructor for ViewPlane
    pub fn new() -> ViewPlane
    {
        ViewPlane{ m_hres: 200, m_vres: 200, m_pixsize: 0.2, m_gamma: 1.0, m_invgamma: 1.0, m_maxdepth: 5, m_numsample: 0 }
    }

    pub fn set_gamma(&mut self, newgamma: f32)
    {
        self.m_gamma = newgamma;
        self.m_invgamma = 1.0 / newgamma;
    }

    //
    pub fn get_coordinate_from_index(&self, i: u16, j: u16) -> Result<Vector2<f32>, &str>
    {
        match self.is_coordinates_valid(i, j)
        {
            false => Err("invalid coordinates"),
            _ => {
                Ok(Vector2::new(self.m_pixsize * (i as f32 - 0.5 * (self.m_hres as f32 - 1.0)),
                                self.m_pixsize * (j as f32 - 0.5 * (self.m_hres as f32 - 1.0))))
            }
        }
    }

    pub fn is_coordinates_valid(&self, i: u16, j: u16) -> bool
    {
        let mut res = true;
        if i < 0 || j < 0 || i >= self.m_hres || j >= self.m_vres
        {
            res = false;
        }
        res
    }
}

#[cfg(test)]
mod ViewPlaneTest
{
    use super::*;

    #[test]
    fn testCoordinateIsValid()
    {
        let mut vp = ViewPlane::new();
        vp.m_hres = 500;
        vp.m_vres = 300;
        assert!(!vp.is_coordinates_valid(501, 14));
    }

    #[test]
    fn testGetCoordinate()
    {
        let mut vp = ViewPlane::new();
        vp.m_hres = 500;
        vp.m_vres = 300;
        vp.m_pixsize = 0.5;
        let coordinate = vp.getCoordinateFromIndex(30, 50);
    }
}