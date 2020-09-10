use crate::brdf::{BRDF, Transmitter};
use crate::utils::shaderec::ShadeRec;
use cgmath::{Vector3, InnerSpace, ElementWise};
use crate::utils::color::Colorf;
use cgmath::num_traits::{Inv, Pow};
use crate::utils::colorconstant::{COLOR_BLACK, COLOR_WHITE};

pub struct FresnelReflector
{
    pub m_index_of_reflection_in: f32,
    pub m_index_of_reflection_out: f32,
    m_cr: Colorf,
    m_kr: f32,
}

impl FresnelReflector
{
    // TODO: properly implement FresnelReflector
    pub fn new() -> FresnelReflector
    {
        FresnelReflector
        {
            m_index_of_reflection_in: 1.0,
            m_index_of_reflection_out: 1.2,
            m_cr: COLOR_WHITE,
            m_kr: 0.5,
        }
    }
}

impl BRDF for FresnelReflector
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf { unimplemented!() }

    /// For computing the direction of the reflected ray
    ///
    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_o: &mut Vector3<f32>, pdf: &mut f32) -> Colorf
    {
        let n_dot_w_o = sr.m_normal.dot(*w_o);
        let new_w_i = -*w_o + sr.m_normal.mul_element_wise(n_dot_w_o * 2.0);
        self.m_cr * (self.m_kr / sr.m_normal.dot(new_w_i))
    }

    /// Reflectance of the material
    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf { unimplemented!() }
}

impl Transmitter for FresnelReflector
{
    fn total_internal_reflection(&self, sr: &ShadeRec) -> bool {
        unimplemented!()
    }

    fn fresnel_reflectance(&self, sr: &ShadeRec) -> f32 {
        let mut normal = sr.m_normal;
        let n_dot_d = -normal.dot(sr.m_ray.m_direction);
        let cos_theta_in = n_dot_d;

        let mut eta = self.m_index_of_reflection_in / self.m_index_of_reflection_out;
        if n_dot_d < 0.0
        {
            normal = -normal;
            eta = eta.inv();
        }
        let cos_theta_t = self.calculate_cos_theta_t(&cos_theta_in, &eta);
        let r_parallel = (eta - cos_theta_in.pow(2.0)) / (eta * cos_theta_in + cos_theta_t);
        let r_perpendicular = (cos_theta_in - eta * cos_theta_t) / (cos_theta_in + eta * cos_theta_t);

        // Fresnel reflectance
        0.5 * (r_parallel.pow(2.0) + r_perpendicular.pow(2.0))
    }
}