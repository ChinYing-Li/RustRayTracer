use cgmath::{Vector3,
             InnerSpace,
             ElementWise};
use std::f32::consts::PI;
use std::ops::Mul;
use std::sync::Arc;

use crate::brdf::BRDF;
use crate::world::shaderec::ShadeRec;
use crate::sampler::{mutijittered::MultiJittered,
                     Sampler};
use crate::utils::color::Colorf;
use crate::utils::colorconstant::{COLOR_BLACK,
                                  COLOR_RED};

const INV_PI: f32 = 1.0 / PI;

#[derive(Clone, Debug)]
pub struct GlossySpecular
{
    pub m_colord: Colorf,
    pub m_colors: Colorf,
    pub m_ks: f32,
    pub m_exp: f32,
    m_kd: f32,
    m_samplerptr: Arc<dyn Sampler>
}

impl GlossySpecular
{
    pub fn new(kd: f32, colord: Colorf, sampler: Arc<dyn Sampler>) -> GlossySpecular
    {
        GlossySpecular
        {
            m_kd: kd,
            m_colord: colord,
            m_ks: 0.0,
            m_colors: COLOR_RED,
            m_exp: 1.0,
            m_samplerptr: sampler,
        }
    }

    pub fn set_sampler(&mut self, sampler: &Arc<dyn Sampler>)
    {
        self.m_samplerptr = sampler.clone();
    }

    pub fn set_kd(&mut self, kd: f32)
    {
        self.m_kd = kd;
    }

    pub fn set_ks(&mut self, ks: f32)
    {
        self.m_ks = ks;
    }

    pub fn set_exponent(&mut self, e: f32)
    {
        self.m_exp = e;
    }
}

impl BRDF for GlossySpecular
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        let normal_dot_w_i = (w_i).dot(sr.m_normal);
        let refl = -(w_i) + 2.0 * normal_dot_w_i * w_o;
        let refl_dot_w_o = refl.dot(w_o);
        let mut res = COLOR_BLACK;

        if refl_dot_w_o > 0.0
        {
            res += self.m_colors * refl_dot_w_o.powf(self.m_exp) * self.m_ks;
        }
        res
    }

    fn sample_func(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_o: &mut Vector3<f32>, pdf: &mut f32) -> Colorf
    {
        let n_dot_w_o = sr.m_normal.normalize().dot(*w_o);
        let reflection_direction: Vector3<f32> = sr.m_normal.mul_element_wise(2.0 * n_dot_w_o) - *w_o;
        let w = reflection_direction.normalize();
        let u = Vector3::new(0.00045, 1.0, 0.00045).cross(w).normalize();
        let v = u.cross(w);

        let sample_point = self.m_samplerptr.as_ref().get_hemisphere_sample();
        *w_i = u.mul_element_wise(sample_point.x )
            + v.mul_element_wise(sample_point.y)
            + w.mul_element_wise(sample_point.z);

        if sr.m_normal.dot(*w_i) < 0.0
        {
            *w_i = -(u.mul_element_wise(sample_point.x )
                - v.mul_element_wise(sample_point.y))
                + w.mul_element_wise(sample_point.z);
        }

        let phong_lobe = reflection_direction.dot(*w_i).powf(self.m_exp);
        *pdf = sr.m_normal.dot(*w_i) * phong_lobe;
        self.m_colors * self.m_ks * phong_lobe
    }

    fn rho(&self, _sr: &ShadeRec, _w_o: Vector3<f32>) -> Colorf
    {
        self.m_colord * self.m_kd
    }
}
