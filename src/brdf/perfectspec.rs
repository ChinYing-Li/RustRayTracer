use crate::brdf::BRDF;
use cgmath::{Vector3, InnerSpace, ElementWise};
use crate::utils::color::Colorf;
use crate::world::shaderec::ShadeRec;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::sampler::mutijittered::MultiJittered;

#[derive(Clone, Debug)]
pub struct PerfectSpecular
{
    pub m_kr: f32,
    pub m_cr: Colorf,
    m_samplerptr: Option<MultiJittered>
}

impl PerfectSpecular
{
    pub fn new(kr: f32, cr: Colorf) -> PerfectSpecular
    {
        PerfectSpecular
        {
            m_kr: kr,
            m_cr: cr,
            m_samplerptr: None,
        }
    }
}

impl BRDF for PerfectSpecular
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        COLOR_BLACK
    }

    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_o: &mut Vector3<f32>, pdf: &mut f32) -> Colorf
    {
        let n_dot_w_o = sr.m_normal.dot(*w_o);
        *w_i = -*w_o + sr.m_normal.mul_element_wise(n_dot_w_o * 2.0);
        self.m_cr * (self.m_kr / sr.m_normal.dot(*w_i))
    }

    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf
    {
        COLOR_BLACK
    }
}