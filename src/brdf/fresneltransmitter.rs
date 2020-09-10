use crate::brdf::{BRDF, Transmitter};
use crate::utils::shaderec::ShadeRec;
use cgmath::Vector3;
use crate::utils::color::Colorf;

pub struct FresnelTransmitter
{
    pub m_index_of_reflection_in: f32,
    pub m_index_of_reflection_out: f32,
}

impl FresnelTransmitter
{

}

impl BRDF for FresnelTransmitter
{
    fn func(&self, sr: &ShadeRec, w_i: Vector3<f32>, w_o: Vector3<f32>) -> Colorf { unimplemented!() }

    /// For computing the direction of the reflected ray
    ///
    fn sampleFunc(&self, sr: &ShadeRec, w_i: &mut Vector3<f32>, w_o: &mut Vector3<f32>, pdf: &mut f32) -> Colorf { unimplemented!() }

    /// Reflectance of the material
    fn rho(&self, sr: &ShadeRec, w_o: Vector3<f32>) -> Colorf { unimplemented!() }
}

impl Transmitter for FresnelTransmitter
{
    fn total_internal_reflection(&self, sr: &ShadeRec) -> bool {
        unimplemented!()
    }

    fn fresnel_reflectance(&self, sr: &ShadeRec) -> f32 {
        unimplemented!()
    }
}