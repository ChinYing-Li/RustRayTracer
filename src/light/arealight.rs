use cgmath::{Vector3, InnerSpace, Zero};
use std::sync::Arc;
use crate::material::Material;
use crate::geometry::Geometry;
use crate::light::Light;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;
use std::f32::INFINITY;
use crate::material::emissive::Emissive;
use crate::utils::colorconstant::COLOR_BLACK;

pub struct AreaLight
{
    pub m_light_normal: Vector3<f32>,
    pub m_w_i: Vector3<f32>,
    pub m_sample_point: Vector3<f32>,
    pub m_materialptr: Arc<Emissive>,
    pub m_geomobject: Arc<dyn Geometry>
}

impl AreaLight
{
    pub fn new(material: Arc<Emissive>, geomobject: Arc<dyn Geometry>)-> AreaLight
    {
        AreaLight
        {
            m_sample_point: Vector3::zero(),
            m_light_normal: Vector3::zero(),
            m_w_i: Vector3::zero(),
            m_materialptr: material,
            m_geomobject: geomobject,
        }
    }

    pub fn get_geometric_factor(&self) -> f32
    {
        unimplemented!()
        //let n_dot_d = -self.m_light_normal.dot(self.m_w_i);

    }
}

impl Light for AreaLight
{
    fn get_direction(&self, sr: &ShadeRec) -> Vector3<f32>
    {
        unimplemented!()
        //self.m_sample_point = self.m_geomobject.sample();
    }

    fn L(&self, sr: &ShadeRec) -> Colorf
    {
        let n_dot_w_i = -self.m_light_normal.dot(self.m_w_i);
        if n_dot_w_i > 0.0 { return self.m_materialptr.get_Le(sr); }
        else { return COLOR_BLACK }
    }

    fn does_cast_shadow(&self) -> bool {
        true
    }

    fn is_in_shadow(&self, sr: &ShadeRec, ray: &Ray) -> bool
    {
        let mut time = INFINITY;
        let time_to_sample_point = (self.m_sample_point - ray.m_origin).dot(ray.m_velocity);
        for object in sr.m_worldptr.clone().unwrap().m_objects.iter()
        {
            if object.lock().unwrap().shadow_hit(ray, &mut time) && time < time_to_sample_point
            {
                return true;
            }
        }
        false
    }

    fn get_type(&self) -> String { String::from("AreaLight") }
}