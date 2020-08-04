use crate::brdf::BRDF;
use cgmath::Vector3;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;

pub struct PerfectSpecular
{

}

impl PerfectSpecular
{
    pub fn new() -> PerfectSpecular
    {
        PerfectSpecular{ }
    }
}

impl BRDF for PerfectSpecular
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        Colorf::new(0.0, 0.0, 0.0)
    }

    fn sampleFunc(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        Colorf::new(0.0, 0.0, 0.0)
    }

    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf
    {
        Colorf::new(0.0, 0.0, 0.0)
    }
}