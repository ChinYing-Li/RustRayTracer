pub mod arealighting;
pub mod raycast;
pub mod whitted;

use std::sync::Arc;

use crate::ray::Ray;
use crate::utils::color::Colorf;
use crate::world::world::World;
use crate::utils::colorconstant::COLOR_BLACK;

const HUGE_VAL_FOR_TIME: f32 = 1e9;

pub trait Tracer
{
    fn trace_ray(&self, worldptr: Arc<World>, ray: &Ray, depth: u16) -> Colorf;
    fn trace_ray_with_time(&self, worldptr: Arc<World>, ray: &Ray, time: &mut f32, depth: u16) -> Colorf
    { COLOR_BLACK }
}

impl std::fmt::Debug for dyn Tracer
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", "Tracer trait object - ")
    }
}