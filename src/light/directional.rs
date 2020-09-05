use cgmath::{Vector3, InnerSpace};
use crate::light::Light;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;

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
        Directional{ m_ls: ls, m_color: color, m_direction: direction.normalize()}
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

impl Light for Directional
{
    fn get_direction(&self, sr: &ShadeRec) -> Vector3<f32>
    {
        self.m_direction
    }

    fn L(&self, sr: &ShadeRec) -> Colorf
    {
        self.m_color * self.m_ls
    }

    fn does_cast_shadow(&self) -> bool { true }

    fn is_in_shadow(&self, sr: &ShadeRec, ray: &Ray) -> bool
    {
        true
    }
}