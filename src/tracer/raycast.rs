use std::f32;
use std::sync::Arc;

use crate::ray::Ray;
use crate::utils::color::{Colorf};
use crate::world::world::World;
use crate::tracer::{Tracer, HUGE_VAL_FOR_TIME};
use crate::utils::colorconstant::COLOR_BLACK;
use crate::tracer::whitted::Whitted;
use crate::world::shaderec::ShadeRec;

pub struct RayCast
{}

impl RayCast
{
    pub fn new() -> RayCast { RayCast{} }
}

impl Tracer for RayCast
{
    // In Raycast, we ignore the recursion depth.
    fn trace_ray(&self, worldptr: Arc<World>, ray: &Ray, depth: u16) -> Colorf
    {
        let mut sr = World::hit_objects(worldptr.clone(), ray, f32::INFINITY);
        if sr.m_hit
        {
            sr.m_ray = *ray;
            if let material = sr.m_material.clone().unwrap()
            {
                return material.shade(&mut sr);
            }
        }
        worldptr.as_ref().m_backgroundcolor
    }

    fn trace_ray_with_time(&self, worldptr: Arc<World>, ray: &Ray, time: &mut f32, depth: u16) -> Colorf
    {
        unimplemented!()
    }
}