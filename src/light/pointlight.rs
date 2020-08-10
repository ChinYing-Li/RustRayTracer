use cgmath::Vector3;
use crate::light::Light;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;

#[derive(Debug)]
pub struct PointLight
{
    m_ls: f32, // Radiance scaling factor
    m_color: Colorf,
    m_location: Vector3<f32>
}

impl PointLight
{
    pub fn new(ls: f32, color: Colorf, location: Vector3<f32>) -> PointLight
    {
        PointLight{ m_ls: ls, m_color: color, m_location: location}
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

impl Light for PointLight
{
    fn getDirection(&self, sr: &mut ShadeRec) -> Vector3<f32>
    {
        (self.m_location - sr.m_hitpoint)
    }

    fn L(&self, sr: &mut ShadeRec) -> Colorf
    {
        self.m_color * self.m_ls
    }
}