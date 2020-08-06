use crate::brdf::lambertian::Lambertian;
use crate::utils::shaderec::ShadeRec;
use std::sync::Arc;

pub struct Matte
{
    m_ambient_brdf: Arc<Lambertian>,
    m_diffuse_brdf: Arc<Lambertian>
}

impl Matte
{
    pub fn new(ambient_brdf: Arc<Lambertian>, diffuse_brdf: Arc<Lambertian>) -> Matte
    {
        Matte{ m_ambient_brdf: ambient_brdf, m_diffuse_brdf: diffuse_brdf}
    }
}
