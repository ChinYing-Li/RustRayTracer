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

