use crate::tracer::Tracer;
use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::ray::Ray;
use crate::world::world::World;
use std::f32::INFINITY;

pub struct AreaLighting
{}

impl AreaLighting
{
    pub fn new() -> AreaLighting
    {
        AreaLighting{}
    }
}

impl Tracer for AreaLighting
{
    fn trace_ray(&self, worldptr: Arc<World>, ray: &Ray, depth: u16) -> Colorf {
        let mut sr = World::hit_objects(worldptr.clone(), ray, INFINITY);
        if sr.m_ishitting
        {
            sr.m_ray = *ray;
            let mat_clone = sr.m_material.clone().unwrap();
            return mat_clone.area_light_shade(&mut sr);
        }

        worldptr.m_backgroundcolor
    }
}