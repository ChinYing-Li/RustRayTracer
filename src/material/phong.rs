use crate::brdf::lambertian::Lambertian;
use std::sync::Arc;
use crate::brdf::glossyspec::GlossySpecular;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::brdf::BRDF;
use crate::light::Light;
use cgmath::{InnerSpace, Vector3, Zero};
use crate::ray::Ray;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::tracer::Tracer;

#[derive(Clone, Debug)]
pub struct Phong
{
    m_ambient_brdf: Arc<Lambertian>,
    m_diffuse_brdf: Arc<Lambertian>,
    m_spec_brdf: Arc<GlossySpecular>
}

impl Phong
{
    pub fn new(ambient_brdf: Arc<Lambertian>, diffuse_brdf: Arc<Lambertian>, spec_brdf: Arc<GlossySpecular>) -> Phong
    {
        Phong
        {
            m_ambient_brdf: ambient_brdf,
            m_diffuse_brdf: diffuse_brdf,
            m_spec_brdf: spec_brdf
        }
    }
}

impl Material for Phong
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf {
        let mut w_o = -sr.m_ray.m_direction.normalize();
        let worldptr = sr.m_worldptr.clone();
        let mut clr = worldptr.m_ambientlight.L(sr)
                            * self.m_ambient_brdf.rho(sr, w_o);

        for light in worldptr.m_lights.iter()
        {
            // TODO handle different shading procedure for different kinds of light
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
                let shadow_ray = Ray::new(sr.m_hitpoint, w_i);
                in_shadow = light.is_in_shadow(sr, &shadow_ray);
            }

            if !in_shadow
            {
                clr += (self.m_diffuse_brdf.func(sr, w_i, w_o) +
                    self.m_spec_brdf.func(sr, w_i, w_o)) *
                    temp_L  * n_dot_w_i;
            }
        }
        clr
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf {
        unimplemented!()
    }

    /*
    fn pathShade<'a>(&self, sr: &'a mut ShadeRec<'a>) {
        unimplemented!()
    }
    */
}