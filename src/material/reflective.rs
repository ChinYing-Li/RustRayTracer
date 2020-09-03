use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use cgmath::Vector3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Reflective
{

}

impl Reflective
{

}

impl Material for Reflective
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        unimplemented!()
    }

    fn area_light_shade<'a>(&self, sr: &'a mut ShadeRec) -> Colorf {
        unimplemented!()
    }
}