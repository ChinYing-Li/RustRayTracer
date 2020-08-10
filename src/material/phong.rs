use crate::brdf::lambertian::Lambertian;
use std::sync::Arc;
use crate::brdf::glossyspec::GlossySpecular;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::brdf::BRDF;
use crate::light::Light;
use cgmath::InnerSpace;

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
        let w_o = -sr.m_ray.m_velocity;
        let mut clr = sr.m_worldref.m_ambientlight.L(sr) * self.m_ambient_brdf.rho(sr, w_o);

        for i in 0..(sr.m_worldref.m_lights.len())
        {
            let w_i = sr.m_worldref.m_lights[i].getDirection(sr);
            let n_dot_w_i = sr.m_normal.dot(sr.m_normal);

            if(n_dot_w_i > 0.0)
            {
                clr += self.m_diffuse_brdf.func(sr, w_o, w_i);
                clr += self.m_spec_brdf.func(sr, w_o, w_i);
                clr *= sr.m_worldref.m_lights[i].L(sr)  * n_dot_w_i;
            }
        }
        clr
    }

    /*
    fn areaLightShade<'a>(&self, sr: &'a mut ShadeRec<'a>) {
        unimplemented!()
    }

    fn pathShade<'a>(&self, sr: &'a mut ShadeRec<'a>) {
        unimplemented!()
    }
    */
}