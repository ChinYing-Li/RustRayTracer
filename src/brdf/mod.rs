pub mod perfecttransmitter;
pub mod fresnelreflector;
pub mod fresneltransmitter;
pub mod glossyspec;
pub mod perfectspec;
pub mod lambertian;

use cgmath::{Vector3};
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::utils::colorconstant::COLOR_BLACK;

pub trait BRDF
{
    /// the Bidirectional Reflectance Distribution Function itself
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf { COLOR_BLACK }

    /// For computing the direction of the reflected ray
    ///
    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_o: &mut Vector3<f32>, pdf: &mut f32) -> Colorf { COLOR_BLACK }

    /// Reflectance of the material
    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf { COLOR_BLACK }
}

pub trait Transmitter
{
    fn total_internal_reflection(&self, sr: &ShadeRec) -> bool;

    fn fresnel_reflectance(&self, sr: &ShadeRec) -> f32;

    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_t: &mut Vector3<f32>) -> Colorf
    { COLOR_BLACK }

    fn calculate_cos_theta_t(&self, cos_theta_in: &f32, eta: &f32) -> f32
    {
        let temp = 1.0 - (1.0 - (*cos_theta_in).powf(2.0)) / (*eta).powf(2.0);
        temp.sqrt()
    }
}

pub trait BTDF: BRDF + Transmitter {}
impl<T> BTDF for T where T: BRDF + Transmitter {}