pub mod glossyspec;
pub mod perfectspec;
pub mod lambertian;

use cgmath::{Vector3};
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;

pub trait BRDF
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf;
    fn sampleFunc(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf;
    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf;
}

