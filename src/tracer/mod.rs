use crate::ray::Ray;
use crate::utils::color::Color;
use crate::world::world::World;
use std::sync::Arc;

pub mod whitted;

pub trait Tracer
{
    fn traceRay(&self, world: &World, ray: &Ray, depth: u16) -> Color;
}

impl std::fmt::Debug for Tracer
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", "Tracer trait object - ")
    }
}