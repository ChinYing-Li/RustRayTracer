use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use cgmath::{Vector3, Zero, InnerSpace};
use crate::ray::Ray;
use crate::material::Material;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::material::phong::Phong;
use crate::tracer::Tracer;
use crate::brdf::perfectspec::PerfectSpecular;
use std::sync::Arc;
use crate::brdf::BRDF;

pub struct Reflective
{
    m_phong: Phong,
    m_reflective_brdf: Arc<PerfectSpecular>,
}

impl Reflective
{

}

impl Material for Reflective
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        let mut clr = self.m_phong.area_light_shade(sr);
        let mut w_o = -sr.m_ray.m_direction;
        let mut w_i = Vector3::zero();
        let mut dummy = 0.0;
        let fr = self.m_reflective_brdf.sampleFunc(sr,&mut w_i, &mut w_o, &mut dummy);
        let reflected_ray = Ray::new(sr.m_hitpoint, w_i);

        clr += fr * sr.m_worldptr.clone().unwrap().m_tracer.trace_ray(sr.m_worldptr.as_ref(), &reflected_ray, sr.m_depth +1)
            * sr.m_normal.dot(w_i);
        clr
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        unimplemented!()
    }
}