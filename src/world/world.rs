use std::sync::Arc;
use std::cell::Cell;
use cgmath::{Vector2, Vector3};
use std::{f32, f32::INFINITY};

use crate::utils::color::Color;
use crate::world::viewplane::ViewPlane;
use crate::output::OutputManager;
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::geometry::Geometry;
use crate::tracer::Tracer;

#[derive(Debug)]
pub struct World
{
    pub m_backgroundcolor: Color,
    pub m_viewplaneptr: Box<ViewPlane>,
    pub m_objects: Vec<Arc<dyn Geometry>>,
    pub m_tracerptr: Box<dyn Tracer>,
}

impl World
{
    pub fn new(viewplane: Box<ViewPlane>, tracer: Box<dyn Tracer>) -> World
    {
        World
        {
            m_backgroundcolor: Color::new(0.0, 0.0, 0.0),
            m_viewplaneptr: viewplane,
            m_objects: Vec::new(),
            m_tracerptr: tracer,
        }
    }

    pub fn setBackgroundColor(&mut self, newColor: Color)
    {
        self.m_backgroundcolor = newColor;
    }

    pub fn build(&mut self)
    {

    }

    pub fn addObject(&mut self, object: Arc<dyn Geometry>)
    {
        self.m_objects.push(object);
    }

    pub fn removeObject(&mut self, index: usize)
    {
        self.m_objects.remove(index);
    }

    pub fn hitObjects<'a>(&'a self, ray: &'a mut Ray, tmin: f32) -> ShadeRec<'a>
    {
        let mut sr = ShadeRec::new(self);
        let srref = &mut sr;
        let mut normal = Vector3::new(0.0, 0.0, 0.0);
        let mut hitpoint = Vector3::new(0.0, 0.0,0.0);
        let mut tglobal = 100000.0;
        let mut tminglobal = tmin;
        let numobj = self.m_objects.len();

        /*let c_updateShadeRec = | time: &mut f32 |
            {
                tminglobal = *time;
                srref.m_ishitting = true;
                srref.m_hitpoint = ray.m_origin + tminglobal * ray.m_velocity;
                normal = srref.m_normal;
                hitpoint = srref.m_hitpoint;
            };
        */
        println!("number of objects: {}", numobj);
        for i in 0..numobj
        {
            if self.m_objects[i].hit(ray, &mut tglobal, srref) && tglobal < tminglobal
            {
                println!("does hit!");
                tminglobal = tglobal;
                srref.m_color = self.m_objects[i].getColor();
                srref.m_ishitting = true;
                srref.m_hitpoint = ray.m_origin + tminglobal * ray.m_velocity;
                normal = srref.m_normal;
                hitpoint = srref.m_hitpoint;
            }
        }

        if sr.m_ishitting
        {
            sr.m_time = tminglobal;
            sr.m_normal = normal;
            sr.m_hitpoint= hitpoint;
        }
        sr
    }

    pub fn renderScence(&self)
    {
        let zdepth = 100.0;
        let mut ray = Ray::new(Vector3::new(0.0, 0.0, -1.0),
                                Vector3::new(0.0, 0.0, 0.0));
        let mut pixcolor = Color::new(0.0, 0.0, 0.0);
        let mut samplecoord = Vector2::new(0.0, 0.0);

        for i in 0..self.m_viewplaneptr.m_hres
        {
            for j in 0..self.m_viewplaneptr.m_vres
            {
                if let Ok(coord) = self.m_viewplaneptr.getCoordinateFromIndex(i, j)
                {
                    ray.m_origin = Vector3::new(coord[0], coord[1], zdepth);
                    pixcolor = self.m_tracerptr.traceRay(&self, &ray, 0); // Not yet implemented tracer!!!
                }
                else
                {
                    println!("Invalid coordinates");
                }
            }
        }
    }

    pub fn writePixel(rownum: i32, colnum:i32, color: Color, opmanager: Arc<dyn OutputManager>)
    {

    }
}

#[cfg(test)]
mod WorldTest
{
    use super::*;
    use crate::geometry::{sphere::Sphere};
    use cgmath::Vector3;

    #[test]
    fn checkAddObject()
    {
       // TODO: Write the test that compare Geometry objects
    }

    #[test]
    fn checkRemoveObject()
    {

    }
}

#[cfg(test)]
mod WorldSphereTest
{
    use super::*;
    use crate::geometry::sphere::Sphere;
    use cgmath::Vector3;
    use crate::ray::Ray;
    use crate::utils::shaderec::ShadeRec;
    use crate::world::world::World;
    use crate::tracer::whitted::Whitted;

    #[test]
    fn checkHitSingleSphere()
    {
        let sphere = Sphere::new(5.0, Vector3::new(0.0, 0.0, 0.0), Color::new(1.0, 0.0, 0.0));
        let boxedtracer = Box::new(Whitted::new());
        let boxedvp = Box::new(ViewPlane::new());
        let mut world = World::new(boxedvp, boxedtracer);
        world.addObject(Arc::new(sphere));

        let mut ray = Ray::new(Vector3::new(10.0, 3.0, 0.0),
                               Vector3::new(-1.0, 0.0, 0.0));
        let mut shaderecord = world.hitObjects(&mut ray, INFINITY);

        assert!(shaderecord.m_ishitting);
        assert_eq!(shaderecord.m_time, 6.0);
        assert_eq!(shaderecord.m_hitpoint, Vector3::new(4.0, 3.0, 0.0));
    }

    #[test]
    fn checkNoHit()
    {
        let sphere = Sphere::new(5.0, Vector3::new(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 1.0));
        let mut ray = Ray::new(Vector3::new(7.0, 0.5, 0.0), Vector3::new(-3.0, 3.0, 0.0));
        let boxedtracer = Box::new(Whitted::new());
        let boxedvp = Box::new(ViewPlane::new());
        let world = World::new(boxedvp, boxedtracer);
        let mut shaderecord = ShadeRec::new(&world);
        let mut tmin = 100.0;
        let res = sphere.hit(&mut ray, &mut tmin, &mut shaderecord);
        assert!(!res);
    }
}