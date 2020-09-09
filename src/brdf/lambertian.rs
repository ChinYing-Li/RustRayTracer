use crate::utils::color::Colorf;
use cgmath::{Vector3, InnerSpace};
use crate::brdf::BRDF;
use std::f32::consts::PI;
use crate::utils::shaderec::ShadeRec;
use crate::sampler::mutijittered::MultiJittered;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::sampler::Sampler;

const INV_PI: f32 = 1.0 / PI;

#[derive(Clone, Debug)]
pub struct Lambertian
{
    m_kd: f32,
    m_colord: Colorf,
    m_samplerptr: Option<MultiJittered>
}

impl Lambertian
{
    pub fn new(kd: f32, colord: Colorf) -> Lambertian
    {
        Lambertian{
            m_kd: kd,
            m_colord: colord,
            m_samplerptr: None,
        }
    }

    fn setup_sampler(num_pattern: usize, sample_per_pattern: usize, e: f32) -> MultiJittered
    {
        let mut sampler = MultiJittered::new(sample_per_pattern, num_pattern);
        sampler.set_map_to_hemisphere(true, e);
        sampler
    }
}

impl BRDF for Lambertian
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        self.m_colord * INV_PI * self.m_kd
    }

    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_o: &mut Vector3<f32>, pdf: &mut f32) -> Colorf
    {
        let w = sr.m_normal.normalize();
        let jittered_up = Vector3::new(0.00034, 0.00012, 1.0);
        let v = jittered_up.cross(w).normalize();
        let u = v.cross(w);

        let sample_point = self.m_samplerptr.as_ref().unwrap().get_hemisphere_sample();
        Colorf::new(0.0, 0.0, 0.0)
    }

    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf
    {
        self.m_colord * self.m_kd
    }
}
