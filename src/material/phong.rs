use crate::brdf::lambertian::Lambertian;
use std::sync::Arc;
use crate::brdf::glossyspec::GlossySpecular;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::brdf::BRDF;
use crate::light::Light;
use cgmath::InnerSpace;
use crate::ray::Ray;
use crate::utils::colorconstant::COLOR_BLACK;

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
        Phong{ m_ambient_brdf: ambient_brdf, m_diffuse_brdf: diffuse_brdf, m_spec_brdf: spec_brdf}
    }
}

impl Material for Phong
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf {
        let mut w_o = -sr.m_ray.m_velocity.normalize();
        let worldptr = sr.m_worldptr.clone().unwrap();
        let mut clr = worldptr.m_ambientlight.L(sr) * self.m_ambient_brdf.rho(sr, w_o);

        for i in 0..(worldptr.m_lights.len())
        {
            let mut w_i = worldptr.m_lights[i].get_direction(sr);
            let n_dot_w_i = sr.m_normal.normalize().dot(w_i);
            println!("n_dot_w_i{}", n_dot_w_i);
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
                    clr += (self.m_diffuse_brdf.func(sr, w_i, w_o) +
                        self.m_spec_brdf.func(sr, w_i, w_o)) *
                        worldptr.m_lights[i].L(sr)  * n_dot_w_i;
                }
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