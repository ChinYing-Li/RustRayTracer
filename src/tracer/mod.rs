use crate::ray::Ray;
use crate::utils::color::Colorf;
use crate::world::world::World;
use std::sync::Arc;
use std::rc::Rc;

pub mod whitted;

pub trait Tracer
{
    fn trace_ray(&self, worldptr: Arc<World>, ray: &Ray, depth: u16) -> Colorf;
}

impl std::fmt::Debug for Tracer
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", "Tracer trait object - ")
    }
}