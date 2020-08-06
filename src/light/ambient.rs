use cgmath::{Vector3, Zero};
use crate::light::Light;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;

#[derive(Debug)]
pub struct Ambient
{
    /// Radiance scaling factor
    m_ls: f32,
    m_color: Colorf,
}

impl Ambient
{
    pub fn new(color: Colorf) -> Ambient
    {
        Ambient{ m_ls: 1.0, m_color: color}
    }

    pub fn setRadianceScalingFactor(&mut self, new_ls: f32)
    {
        self.m_ls = new_ls;
    }

    pub fn setColor(&mut self, newcolor: Colorf)
    {
        self.m_color = newcolor;
    }
}

impl Light<'_> for Ambient
{
    fn getDirection<'a>(&self, sr: &'a mut ShadeRec) -> Vector3<f32>
    {
        Vector3::zero()
    }

    fn L<'a>(&self, sr: &'a mut ShadeRec) -> Colorf
    {
        self.m_color * self.m_ls
    }
}