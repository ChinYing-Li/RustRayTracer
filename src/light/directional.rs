use cgmath::Vector3;
use crate::light::Light;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;

#[derive(Debug)]
pub struct Directional
{
    /// Radiance scaling factor
    m_ls: f32,
    m_color: Colorf,
    m_direction: Vector3<f32>
}

impl Directional
{
    pub fn new(ls: f32, color: Colorf, direction: Vector3<f32>) -> Directional
    {
        Directional{ m_ls: ls, m_color: color, m_direction: direction}
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

impl Light<'_> for Directional
{
    fn getDirection<'a>(&self, sr: &'a mut ShadeRec) -> Vector3<f32>
    {
        self.m_direction
    }

    fn L<'a>(&self, sr: &'a mut ShadeRec) -> Colorf
    {
        self.m_color * self.m_ls
    }
}