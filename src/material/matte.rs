use crate::brdf::lambertian::Lambertian;
use crate::utils::shaderec::ShadeRec;
use std::sync::Arc;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::brdf::BRDF;
use cgmath::InnerSpace;
use crate::light::Light;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::ray::Ray;

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
        let direction = -sr.m_ray.m_velocity.normalize();
        let worldptr = sr.m_worldptr.clone().unwrap();
        let mut res_color = worldptr.m_ambientlight.L(sr) * self.m_ambient_brdf.rho(sr, direction);

        for i in 0..(worldptr.m_lights.len())
        {
            let w_i = worldptr.m_lights[i].get_direction(sr);
            let n_dot_w_i = sr.m_normal.normalize().dot(w_i);

            if n_dot_w_i > 0.0
            {
                let mut in_shadow = false;
                if worldptr.m_lights[i].does_cast_shadow()
                {
                    let shadow_ray = Ray::new(sr.m_hitpoint, w_i);
                    in_shadow = worldptr.m_lights[i].is_in_shadow(sr, &shadow_ray);
                }
                if in_shadow { return COLOR_BLACK }
                res_color += worldptr.m_lights[i].L(sr) * n_dot_w_i * self.m_diffuse_brdf.func(sr, w_i, direction);
            }
        }
        res_color
    }
/* TODO: shading functions for different lights
    fn areaLightShade<'a>(&self, sr: &'a mut ShadeRec);
    fn pathShade<'a>(&self, sr: &'a mut ShadeRec);

 */
}
