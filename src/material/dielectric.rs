use cgmath::{Vector3, Zero, InnerSpace};
use std::f32::INFINITY;
use std::ops::Deref;
use std::sync::Arc;

use crate::utils::colorconstant::COLOR_BLACK;
use crate::utils::color::Colorf;
use crate::world::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::material::{Material,
                      phong::Phong};
use crate::brdf::{BRDF,
                  fresnelreflector::FresnelReflector,
                  fresneltransmitter::FresnelTransmitter,
                  Transmitter};

/// Calls FresnelReflector::sampleFunc, FresnelTransmitter::total_internal_reflection
/// and FresnelTransmitter::sampleFunc
pub struct Dielectric
{
    m_phong: Arc<Phong>,
    m_color_filter_in: Colorf,
    m_color_filter_out: Colorf,
    m_fresnel_brdf: Arc<FresnelReflector>,
    m_fresnel_btdf: Arc<FresnelTransmitter>,
}

impl Dielectric
{
    pub fn new(phong: Arc<Phong>,
               color_filter_in: Colorf,
               color_filter_out: Colorf,
                fresnel_brdf: Arc<FresnelReflector>,
                fresnel_btdf: Arc<FresnelTransmitter>
                ) -> Dielectric
    {
        Dielectric
        {
            m_phong: phong,
            m_color_filter_in: color_filter_in,
            m_color_filter_out: color_filter_out,
            m_fresnel_brdf: fresnel_brdf,
            m_fresnel_btdf: fresnel_btdf
        }
    }
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

        let mut t = INFINITY;
        let mut clr_reflected = COLOR_BLACK;
        let mut clr_transmitted = COLOR_BLACK;
        let n_dot_w_i = sr.m_normal.dot(w_i);

        if self.m_fresnel_brdf.total_internal_reflection(sr)
        {
            clr_reflected = sr.m_worldptr.as_ref().
                m_tracer.trace_ray_with_time(sr.m_worldptr.clone(), &reflected_ray, &mut t, sr.m_depth + 1);
            if n_dot_w_i < 0.0
            {
                // reflected ray is inside
                clr += self.m_color_filter_in.color_filter_exponential(t) * clr_reflected;
            }
            else
            {
                // reeflected ray is outside
                clr += self.m_color_filter_out.color_filter_exponential(t) * clr_reflected;
            }
        }
        else
        {
            // no total internal reflection
            let mut w_t = Vector3::zero();
            let ft = Transmitter::sampleFunc(self.m_fresnel_btdf.as_ref(), sr, &mut w_o, &mut w_t);
            let transmittted_ray = Ray::new(sr.m_hitpoint, w_t);
            let n_dot_w_t = sr.m_normal.dot(w_t);

            if n_dot_w_t < 0.0
            {
                // Reflected ray is inside
                clr_reflected = sr.m_worldptr.as_ref().m_tracer.trace_ray(sr.m_worldptr.clone(), &reflected_ray,  sr.m_depth + 1) * fr * n_dot_w_i.abs();
                clr += clr_reflected * self.m_color_filter_in.color_filter_exponential(t);

                // Transmitted ray is outside
                clr_transmitted = sr.m_worldptr.as_ref().m_tracer.trace_ray(sr.m_worldptr.clone(), &transmittted_ray,sr.m_depth + 1) * ft * n_dot_w_i.abs();
                clr += clr_transmitted * self.m_color_filter_out.color_filter_exponential(t);
            }
            else
            {
                // Reflected ray is outside
                clr_reflected = sr.m_worldptr.as_ref().m_tracer.trace_ray(sr.m_worldptr.clone(), &reflected_ray,  sr.m_depth + 1) * fr * n_dot_w_i.abs();
                clr += clr_reflected * self.m_color_filter_out.color_filter_exponential(t);

                // Transmitted ray is inside
                clr_transmitted = sr.m_worldptr.as_ref().m_tracer.trace_ray(sr.m_worldptr.clone(), &transmittted_ray,sr.m_depth + 1) * ft * n_dot_w_i.abs();
                clr += clr_transmitted * self.m_color_filter_in.color_filter_exponential(t);
            }
        }
        clr
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf {
        unimplemented!()
    }
}