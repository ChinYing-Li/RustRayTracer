use crate::brdf::lambertian::Lambertian;
use crate::utils::shaderec::ShadeRec;
use std::sync::Arc;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::brdf::BRDF;
use cgmath::InnerSpace;
use crate::light::Light;

#[derive(Clone, Debug)]
pub struct Matte
{
    pub(crate) m_ambient_brdf: Arc<Lambertian>,
    pub(crate) m_diffuse_brdf: Arc<Lambertian>
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
        let worldptr = sr.m_worldptr.clone().unwrap();
        let mut res_color = worldptr.m_ambientlight.L(sr) * self.m_ambient_brdf.rho(sr, direction);

        for i in 0..(worldptr.m_lights.len())
        {
            let in_direction = worldptr.m_lights[i].get_direction(sr);
            let in_dot_normal = in_direction.dot(sr.m_normal);

            if in_dot_normal > 0.0
            {
                res_color += worldptr.m_lights[i].L(sr) * in_dot_normal * self.m_diffuse_brdf.func(sr, direction, in_direction);
            }
        }
        res_color
    }
/* TODO: shading functions for different lights
    fn areaLightShade<'a>(&self, sr: &'a mut ShadeRec);
    fn pathShade<'a>(&self, sr: &'a mut ShadeRec);

 */
}
