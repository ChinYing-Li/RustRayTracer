// Just a temporary solution to transparent material

use crate::brdf::{Transmitter, BRDF};
use crate::utils::shaderec::ShadeRec;
use cgmath::{InnerSpace, Vector3};
use cgmath::num_traits::{Inv, Pow};
use crate::utils::color::Colorf;
use crate::utils::colorconstant::COLOR_WHITE;
use std::ops::Neg;

pub struct PerfectTransmitter
{
    m_kt: f32,
    m_index_of_reflection: f32,
}

impl Transmitter for PerfectTransmitter
{
    fn total_internal_reflection(&self, sr: &ShadeRec) -> bool {
        let w_o = -sr.m_ray.m_direction;
        let cos_theta_in = sr.m_normal.dot(w_o);
        let mut eta = self.m_index_of_reflection;

        if cos_theta_in < 0.0
        {
            eta = eta.inv();
        }
        self.calculate_cos_theta_t(&cos_theta_in, &eta) > 0.0
    }

    fn fresnel_reflectance(&self, sr: &ShadeRec) -> f32
    {
        unimplemented!()
    }

    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_t: &mut Vector3<f32>) -> Colorf
    {
        let mut normal = sr.m_normal;
        let mut cos_theta_in = normal.dot(*w_i);
        let mut eta = self.m_index_of_reflection;

        if cos_theta_in < 0.0
        {
            cos_theta_in = -cos_theta_in;
            normal = -normal;
            eta = eta.inv();
        }

        let cos_theta_transmitted = self.calculate_cos_theta_t(&cos_theta_in, &eta);
        *w_t = w_i.neg() / eta - (cos_theta_transmitted - cos_theta_in / eta) * normal;

        COLOR_WHITE * self.m_kt / eta.pow(2.0) / sr.m_normal.dot(*w_i).abs()
    }
}

impl BRDF for PerfectTransmitter
{
    /// the Bidirectional Reflectance Distribution Function itself
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf {
        unimplemented!()
    }

    /// For computing the direction of the reflected ray
    ///
    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_o: &mut Vector3<f32>, pdf: &mut f32) -> Colorf {
        unimplemented!()
    }

    /// Reflectance of the material
    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf { unimplemented!() }
}