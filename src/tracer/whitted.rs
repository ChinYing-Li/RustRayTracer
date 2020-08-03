use crate::ray::Ray;
use crate::utils::color::{Colorf};
use crate::world::world::World;
use std::sync::Arc;
use crate::tracer::Tracer;
use crate::utils::shaderec::ShadeRec;

pub struct Whitted
{
    //
}

impl Whitted
{
    pub fn new() -> Whitted { Whitted{} }
}

impl Tracer for Whitted
{
    fn traceRay(&self, world: &World, ray: &Ray, currentdepth: u16) -> Colorf
    {
        if currentdepth > world.m_viewplaneptr.m_maxdepth
        {
            Colorf::new(0.0, 0.0, 0.0)
        }
        else
        {
            let mut sr = ShadeRec::new(world);
            if sr.m_ishitting
            {
                sr.m_depth = currentdepth;
                sr.m_ray = *ray;
                sr.m_color
            }
            else { world.m_backgroundcolor }
        }

    }
}