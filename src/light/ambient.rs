use cgmath::{Vector3, Zero};
use crate::light::Light;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;

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

    pub fn set_radiance_scaling_factor(&mut self, new_ls: f32)
    {
        self.m_ls = new_ls;
    }

    pub fn set_color(&mut self, newcolor: Colorf)
    {
        self.m_color = newcolor;
    }
}

impl Light for Ambient
{
    fn get_direction(&self, sr: &mut ShadeRec) -> Vector3<f32>
    {
        Vector3::zero()
    }

    fn L(&self, sr: & mut ShadeRec) -> Colorf
    {
        (self.m_color * self.m_ls).clamp()
    }

    fn does_cast_shadow(&self) -> bool { false }

    fn is_in_shadow(&self, sr: &ShadeRec, ray: &Ray) -> bool {
        false
    }
}