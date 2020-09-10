use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::material::Material;
use crate::utils::shaderec::ShadeRec;
use crate::material::phong::Phong;
use crate::brdf::fresnelreflector::FresnelReflector;
use crate::brdf::fresneltransmitter::FresnelTransmitter;
use cgmath::{Vector3, Zero, InnerSpace};
use crate::brdf::{BRDF, Transmitter};
use crate::ray::Ray;
use std::f32::INFINITY;
use crate::utils::colorconstant::COLOR_BLACK;

pub struct Dielectric
{
    m_phong: Phong,
    m_colorfilter_in: Colorf,
    m_color_filter_out: Colorf,
    m_fresnel_brdf: Arc<FresnelReflector>,
    m_fresnel_btdf: Arc<FresnelTransmitter>,
}

impl Material for Dielectric
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf {
        let mut  clr = self.m_phong.shade(sr);
        let mut w_i = Vector3::zero();
        let mut w_o = -sr.m_ray.m_direction;
        let mut dummy = 0.0_f32;
        let fr = BRDF::sampleFunc(self.m_fresnel_brdf.as_ref(), sr, &mut w_i, &mut w_o, &mut dummy);
        let reflected_ray = Ray::new(sr.m_hitpoint, w_i);

        let t = INFINITY;
        let mut clr_reflected = COLOR_BLACK;
        let mut clr_transmitted = COLOR_BLACK;
        let n_dot_w_i = sr.m_normal.dot(w_i);
        let reflected_color = COLOR_BLACK;
        let transmitted_color = COLOR_BLACK;

        if self.m_fresnel_brdf.total_internal_reflection(sr)
        {
            if n_dot_w_i < 0.0
            {
                // reflected ray is inside???
                //reflected_color = sr.m_worldptr.clone().unwrap().m_tracer.
            }
        }
        COLOR_BLACK
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf {
        unimplemented!()
    }
}