use std::sync::Arc;
use cgmath::{Vector2, Vector3};
use std::{f32};

use crate::utils::color::Colorf;
use crate::world::viewplane::ViewPlane;
use crate::output::OutputManager;
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::geometry::Geometry;
use crate::tracer::Tracer;
use crate::light::ambient::Ambient;
use crate::light::Light;

#[derive(Debug)]
pub struct World
{
    pub m_backgroundcolor: Colorf,
    pub m_viewplaneptr: Box<ViewPlane>,
    pub m_objects: Vec<Arc<dyn Geometry>>,
    pub m_tracerptr: Box<dyn Tracer>,
    pub m_ambientlight: Arc<Ambient>,
    pub m_lights: Vec<Arc<dyn Light>>,
    m_outputmgr: Box<dyn OutputManager>,
}

impl World
{
    pub fn new(viewplane: Box<ViewPlane>, tracer: Box<dyn Tracer>, outmgr: Box<dyn OutputManager>) -> World
    {
        World
        {
            m_backgroundcolor: Colorf::new(0.0, 0.0, 0.0),
            m_viewplaneptr: viewplane,
            m_objects: Vec::new(),
            m_tracerptr: tracer,
            m_ambientlight: Arc::new(Ambient::new(Colorf::new(1.0 , 1.0, 1.0))),
            m_lights: Vec::with_capacity(30),
            m_outputmgr: outmgr,
        }
    }

    pub fn setBackgroundColor(&mut self, newColor: Colorf)
    {
        self.m_backgroundcolor = newColor;
    }

    pub fn build(&mut self)
    {
        // Not following the book
    }

    pub fn addObject(&mut self, object: Arc<dyn Geometry>)
    {
        self.m_objects.push(object);
    }

    pub fn removeObject(&mut self, index: usize)
    {
        self.m_objects.remove(index);
    }

    pub fn addLight(&mut self, light: Arc<dyn Light>)
    {
        self.m_lights.push(light);
    }

    pub fn removeLight(&mut self, index: usize)
    {
        self.m_lights.remove(index);
    }

    pub fn setAmbient(&mut self, ambient: Arc<Ambient>)
    {
        self.m_ambientlight = ambient;
    }

    pub fn writePixel(&mut self, x: u32, y: u32, color: Colorf)
    {
        self.m_outputmgr.writePixel(x, y, color);
    }

    pub fn output(&mut self) { self.m_outputmgr.output(); }

    pub fn hitObjects(&self, ray: &Ray, tmin: f32) -> ShadeRec
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

    pub fn renderScene(&self, output: Box<dyn OutputManager>)
    {
        let zdepth = 100.0;
        let mut ray = Ray::new(Vector3::new(0.0, 0.0, -1.0),
                                Vector3::new(0.0, 0.0, 0.0));
        let mut pixcolor = Colorf::new(0.0, 0.0, 0.0);
        let mut out = output;
        for i in 0..self.m_viewplaneptr.m_hres
        {
            for j in 0..self.m_viewplaneptr.m_vres
            {
                if let Ok(coord) = self.m_viewplaneptr.getCoordinateFromIndex(i, j)
                {
                    ray.m_origin = Vector3::new(coord[0], coord[1], zdepth);
                    pixcolor = self.m_tracerptr.traceRay(&self, &ray, 0); // Not yet implemented tracer!!!
                    (*out).writePixel(i.into(), j.into(), pixcolor);
                }
                else
                {
                    println!("Invalid coordinates");
                }
            }
        }
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
    use cgmath::Vector3;
    use std::f32::INFINITY;

    use super::*;
    use crate::geometry::sphere::Sphere;
    use crate::ray::Ray;
    use crate::utils::shaderec::ShadeRec;
    use crate::world::world::World;
    use crate::tracer::whitted::Whitted;
    use crate::utils::colorconstant::{COLOR_BLUE, COLOR_RED};
    use crate::output::imagewriter::ImageWriter;

    fn setUpDummyWorld() -> World
    {
        let tracer = Box::new(Whitted::new());
        let mut boxed_vp = Box::new(ViewPlane::new());
        let mut imgwriter = Box::new(ImageWriter::new("filedest", 100, 100));

        World::new(boxed_vp, tracer, imgwriter)
    }

    const sphere: Sphere = Sphere{ m_radius: 5.0,
        m_center: Vector3::new(0.0, 0.0, 0.0),
        m_color: COLOR_RED};

    #[test]
    fn checkHitSingleSphere()
    {
        let mut world = setUpDummyWorld();
        world.addObject(Arc::new(sphere));

        let mut ray = Ray::new( Vector3::new(10.0, 3.0, 0.0),
                                Vector3::new(-1.0, 0.0, 0.0));
        let mut shaderecord = world.hitObjects(&ray, INFINITY);

        assert!(shaderecord.m_ishitting);
        assert_eq!(shaderecord.m_time, 6.0);
        assert_eq!(shaderecord.m_hitpoint, Vector3::new(4.0, 3.0, 0.0));
    }

    #[test]
    fn checkNoHit()
    {
        let mut ray = Ray::new(Vector3::new(7.0, 0.5, 0.0), Vector3::new(-3.0, 3.0, 0.0));
        let mut world = setUpDummyWorld();
        let mut shaderecord = ShadeRec::new(&world);
        let mut tmin = 100.0;
        let res = sphere.hit(&ray, &mut tmin, &mut shaderecord);
        assert!(!res);
    }
}