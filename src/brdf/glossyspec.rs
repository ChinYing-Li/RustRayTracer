use crate::utils::color::Colorf;
use cgmath::{Vector3, InnerSpace};
use crate::brdf::BRDF;
use crate::utils::shaderec::ShadeRec;
use std::f32::consts::PI;
use crate::utils::colorconstant::COLOR_BLACK;

const INV_PI: f32 = 1.0 / PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlossySpecular
{
    pub m_kd: f32,
    pub m_colord: Colorf,
    pub m_ks: f32,
    pub m_colors: Colorf,
    pub m_exp: f32,
}

impl GlossySpecular
{
    pub fn new(kd: f32, colord: Colorf) -> GlossySpecular
    {
        GlossySpecular{
            m_kd: kd,
            m_colord: colord,
            m_ks: 0.0,
            m_colors: COLOR_BLACK,
            m_exp: 2.0
        }
    }
}

impl BRDF for GlossySpecular
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        let normal_dot_w_i = w_i.dot(sr.m_normal);
        let refl = -w_i + 2.0 * normal_dot_w_i * w_o;
        let refl_dot_w_o = refl.dot(w_o);
        let mut res = COLOR_BLACK;

        if refl_dot_w_o > 0.0
        {
            res += self.m_colors * refl_dot_w_o.powf(self.m_exp) * self.m_ks;
        }
        res
    }

    fn sampleFunc(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        COLOR_BLACK
    }

    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf
    {
        self.m_colord * self.m_kd
    }
}
