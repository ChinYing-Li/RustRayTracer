use cgmath::{InnerSpace, Vector3, Zero};
use std::sync::Arc;
use std::any::type_name;

use crate::brdf::lambertian::Lambertian;
use crate::world::shaderec::ShadeRec;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::brdf::BRDF;
use crate::light::Light;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::ray::Ray;
use crate::tracer::Tracer;

#[derive(Clone, Debug)]
pub struct Matte
{
    pub m_ambient_brdf: Arc<Lambertian>,
    pub m_diffuse_brdf: Arc<Lambertian>
}

impl Matte
{
    pub fn new(ambient_brdf: Arc<Lambertian>, diffuse_brdf: Arc<Lambertian>) -> Matte
    {
        Matte{ m_ambient_brdf: ambient_brdf, m_diffuse_brdf: diffuse_brdf}
    }

    pub fn set_ka(&mut self, ka: f32)
    {
        Arc::get_mut(&mut self.m_ambient_brdf).unwrap().set_kd(ka);
    }

    pub fn set_kd(&mut self, kd: f32)
    {
        Arc::get_mut(&mut self.m_diffuse_brdf).unwrap().set_kd(kd);
    }

    pub fn set_cd(&mut self, clr: Colorf)
    {

    }
}

impl Material for Matte
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        let direction = -sr.m_ray.m_direction.normalize();
        let worldptr = sr.m_worldptr.clone();
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
                if !in_shadow
                {
                    res_color += worldptr.m_lights[i].L(sr) * n_dot_w_i * self.m_diffuse_brdf.func(sr, w_i, direction);
                }
            }
        }
        res_color
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        let w_o = -sr.m_ray.m_direction.normalize();
        let mut clr = sr.m_worldptr.clone().m_ambientlight.L(sr)
            * self.m_ambient_brdf.rho(sr, w_o);

        for light in sr.m_worldptr.as_ref().m_lights.iter()
        {
            let mut w_i = Vector3::zero();
            let mut n_dot_w_i = 0.0_f32;
            let mut temp_L = light.L(sr);

            w_i = light.get_direction(sr);
            n_dot_w_i = sr.m_normal.normalize().dot(w_i);
            println!("n_dot_w_i{}", n_dot_w_i);
            if n_dot_w_i < 0.0 { continue; }

            let mut in_shadow = true;
            if light.does_cast_shadow()
            {
                let shadow_ray= Ray::new(sr.m_hitpoint, w_i);
                in_shadow = light.is_in_shadow(sr, &shadow_ray);
            }

            if !in_shadow
            {
                clr += self.m_diffuse_brdf.func(sr, w_i, w_o) * temp_L ;
            }
        }
        clr
    }
    /* TODO: shading functions for different lights
        fn pathShade<'a>(&self, sr: &'a mut ShadeRec);

     */
}
