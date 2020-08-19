use crate::ray::Ray;
use crate::utils::color::{Colorf};
use crate::world::world::World;
use std::sync::Arc;
use crate::tracer::Tracer;
use crate::utils::shaderec::ShadeRec;
use std::f32::INFINITY;
use crate::utils::colorconstant::COLOR_BLACK;
use std::rc::Rc;

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
    fn traceRay(&self, worldptr: Rc<World>, ray: &Ray, currentdepth: u16) -> Colorf
    {
        let worldptr_cloned = worldptr.clone();
        if currentdepth > worldptr_cloned.m_viewplaneptr.m_maxdepth
        {
            COLOR_BLACK
        }
        else
        {
            let mut sr = World::hitObjects(worldptr, ray, INFINITY);
            if sr.m_ishitting
            {
                sr.m_depth = currentdepth;
                sr.m_ray = *ray;
                let mat_clone = sr.m_material.clone();
                mat_clone.unwrap().shade(&mut sr)
            }
            else { worldptr_cloned.m_backgroundcolor }
        }

    }
}

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

    fn setUpDummyWorld() -> World
    {
        let tracer = Box::new(Whitted::new());
        let mut boxed_vp = Box::new(ViewPlane::new());
        let mut imgwriter = Box::new(ImageWriter::new("filedest", 100, 100));

        World::new(boxed_vp, tracer, imgwriter)
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
        world.m_viewplaneptr.m_numsample = 3;
        world.addObject(Arc::new(sphereA));

        let ray = Ray::new(Vector3::new(50.0, 30.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let res_color = world.m_tracerptr.traceRay(&world, &ray, 0);
        assert_relative_eq!(res_color.m_r, 1.0);
    }

    #[test]
    fn HitNoSphereTest()
    {
        let mut world = setUpDummyWorld();
        world.m_viewplaneptr.m_pixsize = 0.5;
        world.m_viewplaneptr.m_numsample = 3;
        world.addObject(Arc::new(sphereA));

        let ray = Ray::new(Vector3::new(90.0, 10.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let res_color = world.m_tracerptr.traceRay(&world, &ray, 0);
        assert_relative_eq!(res_color.m_r, 0.0);
    }
}