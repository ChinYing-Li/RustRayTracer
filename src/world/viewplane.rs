use std::{f32};

#[derive(Debug)]
pub struct ViewPlane
{
    pub m_hres: u8,
    pub m_vres: u8,
    pub m_pixsize: f32,
    pub m_gamma: f32,
    pub m_invgamma: f32,
}

impl ViewPlane
{
    /// Default constructor for ViewPlane
    pub fn new() -> ViewPlane
    {
        ViewPlane{ m_hres: 200, m_vres: 200, m_pixsize: 0.2, m_gamma: 1.0, m_invgamma: 1.0}
    }

    pub fn setHRes(&mut self, newres: u8)
    {
        self.m_hres = newres;
    }

    pub fn setPixSize(&mut self, newpixsize: f32)
    {
        self.m_pixsize = newpixsize;
    }

    pub fn setGamma(&mut self, newgamma: f32)
    {
        self.m_gamma = newgamma;
        self.m_invgamma = 1.0 / newgamma;
    }
}

#[cfg(test)]
mod ViewPlaneTest
{

}