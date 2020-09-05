use crate::material::phong::Phong;
use crate::brdf::glossyspec::GlossySpecular;
use std::sync::Arc;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;

pub struct GlossyReflector
{
    pub m_phong: Phong,
    pub m_glossyspec_brdf: Arc<GlossySpecular>
}

impl GlossyReflector
{
    pub fn new(phong: &Phong, glossyspec_brdf: Arc<GlossySpecular>) -> GlossyReflector
    {
        GlossyReflector
        {
            m_phong: (*phong).clone(),
            m_glossyspec_brdf: glossyspec_brdf
        }
    }
}

impl Material for GlossyReflector
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf {
        unimplemented!()
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf {
        unimplemented!()
    }
}