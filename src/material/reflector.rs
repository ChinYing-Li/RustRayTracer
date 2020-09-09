use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use cgmath::Vector3;
use crate::ray::Ray;
use crate::material::Material;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::material::phong::Phong;
use crate::tracer::Tracer;

pub struct Reflective
{
    m_phong: Phong,
    
}

impl Reflective
{

}

impl Material for Reflective
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        /*let mut clr = COLOR_BLACK;
        let w_o = -sr.m_ray.m_direction;
        */
        COLOR_BLACK
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        unimplemented!()
    }
}