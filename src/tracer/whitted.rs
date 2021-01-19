use std::f32;
use std::sync::Arc;

use crate::ray::Ray;
use crate::utils::color::{Colorf};
use crate::world::world::World;
use crate::tracer::{Tracer, HUGE_VAL_FOR_TIME};
use crate::utils::colorconstant::COLOR_BLACK;

pub struct Whitted
{
    // doesn't hold any data
}

impl Whitted
{
    pub fn new() -> Whitted { Whitted{} }
}

impl Tracer for Whitted
{
    fn trace_ray(&self, worldptr: Arc<World>, ray: &Ray, depth: u16) -> Colorf
    {
        if depth > worldptr.as_ref().m_viewplaneptr.m_maxdepth
        {
            COLOR_BLACK
        }
        else
        {
            let mut sr = World::hit_objects(worldptr.clone(), ray, f32::INFINITY);
            if sr.m_hit
            {
                sr.m_depth = depth;
                sr.m_ray = *ray;
                if let material = sr.m_material.clone().unwrap()
                {
                    return material.shade(&mut sr);
                }
            }
            worldptr.as_ref().m_backgroundcolor
        }
    }

    fn trace_ray_with_time(&self, worldptr: Arc<World>, ray: &Ray, time: &mut f32, depth: u16) -> Colorf
    {
        if depth > worldptr.as_ref().m_viewplaneptr.m_maxdepth
        {
            COLOR_BLACK
        }
        else
        {
            let mut sr = World::hit_objects(worldptr.clone(), ray, f32::INFINITY);
            if sr.m_hit
            {
                sr.m_depth = depth;
                sr.m_ray = *ray;
                *time = sr.m_time;
                sr.m_material.clone()
                    .map(|material|  material.shade(&mut sr)).unwrap()
            }
            else
            {
                *time = HUGE_VAL_FOR_TIME;
                worldptr.as_ref().m_backgroundcolor
            }
        }
    }
}
/*
#[cfg(test)]
mod WhittedTest
{
    use super::*;

    use approx::{assert_relative_eq};
    use crate::geometry::sphere::Sphere;
    use cgmath::Vector3;
    use crate::world::viewplane::ViewPlane;
    use crate::output::imagewriter::ImageWriter;
    use crate::utils::colorconstant::{COLOR_RED, COLOR_BLACK};
    use std::sync::Mutex;
    use crate::geometry::Shadable;
    use crate::sampler::mutijittered::MultiJittered;

    fn setUpDummyWorld() -> World
    {
        let sampler = MultiJittered::new(16, 3);
        let mut boxed_vp = Box::new(ViewPlane::new(Arc::new(sampler)));
        World::new(boxed_vp)
    }

    const sphereA: Sphere = Sphere
    {
        m_radius: 30.0,
        m_center: Vector3::new(70.0, 30.0, 20.0),
        m_color: COLOR_RED,
        m_material: None
    };

    #[test]
    fn HitOneSphereTest()
    {
        let mut world = setUpDummyWorld();
        world.m_viewplaneptr.m_pixsize = 0.5;
        sphereA.set_material();
        world.add_object(Arc::new(Mutex::new(sphereA)));

        let ray = Ray::new(Vector3::new(50.0, 30.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let tracer = Box::new(Whitted::new());
        let res_color = tracer.trace_ray(Arc::new(world), &ray, 0);
        assert_relative_eq!(res_color.m_r, 1.0);
    }

    #[test]
    fn HitNoSphereTest()
    {
        let mut world = setUpDummyWorld();
        world.m_viewplaneptr.m_pixsize = 0.5;
        world.m_viewplaneptr.m_numsample = 3;
        world.add_object(Arc::new(Mutex::new(sphereA)));

        let tracer = Box::new(Whitted::new());
        let ray = Ray::new(Vector3::new(90.0, 10.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let res_color = tracer.trace_ray(Arc::new(world), &ray, 0);
        assert_relative_eq!(res_color.m_r, 0.0);
    }
}*/