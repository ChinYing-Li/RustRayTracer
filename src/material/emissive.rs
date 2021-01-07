use crate::material::Material;
use crate::world::shaderec::ShadeRec;
use crate::utils::color::Colorf;
use cgmath::InnerSpace;
use crate::utils::colorconstant::COLOR_BLACK;

pub struct Emissive
{
    m_ls: f32,
    m_color: Colorf,
}

impl Emissive
{
    pub fn new(ls: f32, color: Colorf) -> Emissive
    {
        Emissive
        {
            m_ls: ls,
            m_color: color,
        }
    }

    pub fn scale_radiance(&mut self, ls: f32)
    {
        self.m_ls = ls;
    }

    pub fn set_color(&mut self, emissive_color: Colorf)
    {
        self.m_color = emissive_color;
    }

    pub fn get_Le(&self, sr: &ShadeRec) -> Colorf
    {
        self.m_color * self.m_ls
    }
}

impl Material for Emissive
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf {
        return if -sr.m_normal.dot(sr.m_ray.m_direction) > 0.0
        {
            self.m_color * self.m_ls
        } else { COLOR_BLACK }
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf {
        if -sr.m_normal.dot(sr.m_ray.m_direction) > 0.0
        {
            return self.m_color * self.m_ls;
        }
        else { return COLOR_BLACK }
    }
}