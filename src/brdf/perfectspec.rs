use crate::brdf::BRDF;
use cgmath::Vector3;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::utils::colorconstant::COLOR_BLACK;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PerfectSpecular
{
    // TODO: Perfect Specular BRDF
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
        COLOR_BLACK
    }

    fn sampleFunc(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf
    {
        COLOR_BLACK
    }

    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf
    {
        COLOR_BLACK
    }
}