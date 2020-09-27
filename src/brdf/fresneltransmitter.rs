use crate::brdf::{BRDF, Transmitter};
use crate::world::shaderec::ShadeRec;
use cgmath::{Vector3, InnerSpace};
use crate::utils::color::Colorf;
use cgmath::num_traits::{Inv, Pow};
use std::ops::Neg;
use crate::utils::colorconstant::COLOR_WHITE;

pub struct FresnelTransmitter
{
    pub m_index_of_reflection_in: f32,
    pub m_index_of_reflection_out: f32,
}

impl FresnelTransmitter
{
    pub fn new(index_of_reflection_in: f32, index_of_reflection_out: f32) -> FresnelTransmitter
    {
        FresnelTransmitter
        {
            m_index_of_reflection_in: index_of_reflection_in,
            m_index_of_reflection_out: index_of_reflection_out,
        }
    }
}

impl BRDF for FresnelTransmitter
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf { unimplemented!() }

    /// For computing the direction of the reflected ray
    ///
    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_o: &mut Vector3<f32>, pdf: &mut f32) -> Colorf
    {
        unimplemented!()
    }

    /// Reflectance of the material
    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf { unimplemented!() }
}

impl Transmitter for FresnelTransmitter
{
    // PerfectTransmitter tir
    fn total_internal_reflection(&self, sr: &ShadeRec) -> bool {
        let w_o = -sr.m_ray.m_direction;
        let cos_theta_in = sr.m_normal.dot(w_o);
        let mut eta = self.m_index_of_reflection_in / self.m_index_of_reflection_out;

        if cos_theta_in < 0.0
        {
            eta = eta.inv();
        }
        self.calculate_cos_theta_t(&cos_theta_in, &eta) > 0.0
    }

    fn fresnel_reflectance(&self, sr: &ShadeRec) -> f32 {
        unimplemented!()
    }

    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_t: &mut Vector3<f32>) -> Colorf
    {
        let mut normal = sr.m_normal;
        let mut cos_theta_in = normal.dot(*w_i);
        let mut eta = self.m_index_of_reflection_in / self.m_index_of_reflection_out;

        if cos_theta_in < 0.0
        {
            cos_theta_in = -cos_theta_in;
            normal = -normal;
            eta = eta.inv();
        }

        let cos_theta_t = self.calculate_cos_theta_t(&cos_theta_in, &eta);
        *w_t = w_i.neg() / eta - (cos_theta_t - cos_theta_in) * normal / eta;

        let r_parallel = (eta * cos_theta_in - cos_theta_t) / (eta * cos_theta_in + cos_theta_t);
        let r_perpendicular = (cos_theta_in - eta * cos_theta_t) / (cos_theta_in + eta * cos_theta_t);
        let kt = 1.0 - 0.5 * (r_parallel.pow(2.0) + r_perpendicular.pow(2.0));
        COLOR_WHITE * kt / eta.pow(2.0) / sr.m_normal.dot(*w_i).abs()
    }
}