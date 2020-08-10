use crate::brdf::lambertian::Lambertian;
use crate::utils::shaderec::ShadeRec;
use std::sync::Arc;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::brdf::BRDF;
use cgmath::InnerSpace;
use crate::light::Light;

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

impl Material for Matte
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        let direction = -sr.m_ray.m_velocity;
        let mut res_color = sr.m_worldref.m_ambientlight.L(sr) * self.m_ambient_brdf.rho(sr, direction);

        for i in 0..(sr.m_worldref.m_lights.len())
        {
            let in_direction = sr.m_worldref.m_lights[i].getDirection(sr);
            let in_dot_normal = in_direction.dot(sr.m_normal);

            if in_dot_normal > 0.0
            {
                res_color += sr.m_worldref.m_lights[i].L(sr) * in_dot_normal * self.m_diffuse_brdf.func(sr, direction, in_direction);
            }
        }
        res_color
    }
/*
    fn areaLightShade<'a>(&self, sr: &'a mut ShadeRec);
    fn pathShade<'a>(&self, sr: &'a mut ShadeRec);

 */
}
