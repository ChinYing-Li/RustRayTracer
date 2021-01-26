use std::f32;
use std::sync::Arc;

use crate::ray::Ray;
use crate::utils::color::{Colorf};
use crate::world::world::World;
use crate::tracer::{Tracer, HUGE_VAL_FOR_TIME};

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
    fn trace_ray(&self, worldptr: Arc<World>, ray: &Ray, depth: u16) -> Colorf
    {
        let mut sr = World::hit_objects(worldptr.clone(), ray, f32::INFINITY);
        if sr.m_hit
        {
            sr.m_ray = *ray;
            let mat_clone = sr.m_material.clone().unwrap();
            return mat_clone.area_light_shade(&mut sr);
        }
        worldptr.m_backgroundcolor
    }
}