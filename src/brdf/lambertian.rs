use crate::utils::color::Colorf;
use cgmath::Vector3;
use crate::brdf::BRDF;
use std::f32::consts::PI;
use crate::utils::shaderec::ShadeRec;

const INV_PI: f32 = 1.0 / PI;

struct Lambertian
{
    m_kd: f32,
    m_colord: Colorf,
}

impl Lambertian
{
    pub fn new(kd: f32, colord: Colorf) -> Lambertian
    {
        Lambertian{ m_kd: kd, m_colord: colord}
    }
}

impl BRDF for Lambertian
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        self.m_colord * INV_PI * self.m_kd
    }

    fn sampleFunc(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        Colorf::new(0.0, 0.0, 0.0)
    }

    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf
    {
        self.m_colord * self.m_kd
    }
}
