use std::sync::{Arc, Mutex};
use cgmath::{Vector3, Zero};
use std::{f32};

use crate::utils::color::Colorf;
use crate::world::viewplane::ViewPlane;
use crate::world::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::geometry::{Geometry, Shadable, Concrete};
use crate::light::ambient::Ambient;
use crate::light::Light;
use crate::tracer::Tracer;
use crate::tracer::whitted::Whitted;
use crate::tracer::raycast::RayCast;
use crate::utils::colorconstant::COLOR_WHITE;

#[derive(Debug)]
pub struct World
{
    pub m_backgroundcolor: Colorf,
    pub m_viewplaneptr: Box<ViewPlane>,
    pub m_objects: Vec<Arc<Mutex<dyn Concrete>>>,
    pub m_ambientlight: Arc<dyn Light>,
    pub m_lights: Vec<Arc<dyn Light>>,
    pub m_tracer: Arc<dyn Tracer>, // TODO: Holy crap, we don't even use member tracer in World
}

impl World
{
    pub fn new(viewplane: Box<ViewPlane>, tracer: &str) -> World
    {
        World
        {
            m_backgroundcolor: Colorf::new(0.0, 0.0, 0.0),
            m_viewplaneptr: viewplane,
            m_objects: Vec::new(),
            m_ambientlight: Arc::new(Ambient::new(COLOR_WHITE)),
            m_lights: Vec::with_capacity(30),
            m_tracer: World::get_tracer(tracer),
        }
    }

    pub fn set_background_color(&mut self, newColor: Colorf)
    {
        self.m_backgroundcolor = newColor;
    }

    pub fn build(&mut self)
    {
        // Not following the book
    }

    /// Using Mutex as we might want to mutate the underlying object... Or do we?
    pub fn add_object(&mut self, object: Arc<Mutex<dyn Concrete>>)
    {
        self.m_objects.push(object);
    }

    pub fn remove_object(&mut self, index: usize)
    {
        self.m_objects.remove(index);
    }

    pub fn add_light(&mut self, light: Arc<dyn Light>)
    {
        self.m_lights.push(light);
    }

    pub fn remove_light(&mut self, index: usize)
    {
        self.m_lights.remove(index);
    }

    pub fn set_ambient(&mut self, ambient: Arc<dyn Light>)
    {
        self.m_ambientlight = ambient;
    }

    pub fn hit_objects(worldptr: Arc<World>, ray: &Ray, tmin: f32) -> ShadeRec
    {
        let mut sr = ShadeRec::new(worldptr.clone());
        let srref = &mut sr;
        let mut normal = Vector3::zero();
        let mut hitpoint = Vector3::zero();
        let mut local_hitpoint = Vector3::zero();
        let mut tglobal = 100000.0;
        let mut tminglobal = tmin;

        for object in &worldptr.as_ref().m_objects
        {
            if let x = object.lock().unwrap()
            {
                if x.hit(ray, &mut tglobal, srref).unwrap_or(false) && tglobal < tminglobal
                {
                    println!("does hit!");
                    tminglobal = tglobal;
                    srref.m_material = Some(x.get_material());
                    srref.m_hit = true;
                    srref.m_hitpoint = ray.m_origin + tminglobal * ray.m_direction;
                    normal = srref.m_normal;
                    hitpoint = srref.m_hitpoint;
                    local_hitpoint = srref.m_local_hitpoint;
                }
            }
        }

        if sr.m_hit
        {
            sr.m_time = tminglobal;
            sr.m_normal = normal;
            sr.m_hitpoint= hitpoint;
        }
        sr
    }

    pub fn get_dummy() -> World
    {
        World::new(Box::new(ViewPlane::get_dummy()), "whitted")
    }

    fn get_tracer(name: &str) -> Arc<dyn Tracer>
    {
        return match name
        {
            "whitted" => Arc::new(Whitted::new()),
            _ => Arc::new(RayCast::new())
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
    fn check_add_object()
    {
       // TODO: Write the test that compare Geometry objects
    }

    #[test]
    fn check_remove_object()
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
    use crate::world::shaderec::ShadeRec;
    use crate::world::world::World;
    use crate::tracer::whitted::Whitted;
    use crate::utils::colorconstant::{COLOR_BLUE, COLOR_RED};
    use crate::output::imagewriter::ImageWriter;
    use crate::material::matte::Matte;
    use crate::brdf::lambertian::Lambertian;
    use crate::sampler::mutijittered::MultiJittered;

    fn set_up_dummy_world() -> World
    {
        let tracer = Box::new(Whitted::new());
        let mut sampler = MultiJittered::new(256, 1);
        let mut boxed_vp = Box::new(ViewPlane::new(Arc::new(sampler)));
        let mut imgwriter = Box::new(ImageWriter::new("filedest", 100, 100));

        World::new(boxed_vp, "whitted")
    }

    fn setUpSphere() -> Sphere
    {
        let matte = Matte::new(
                            Arc::new(Lambertian::new(2.0, COLOR_RED)),
                            Arc::new(Lambertian::new(1.0, COLOR_RED)), );
        let mut sphere = Sphere::new(5.0, Vector3::new(0.0, 0.0, 0.0));
        sphere.set_material(Arc::new(matte));
        sphere
    }

    #[test]
    fn checkHitSingleSphere()
    {
        let mut world = set_up_dummy_world();
        world.add_object(Arc::new(Mutex::new(setUpSphere())));

        let mut ray = Ray::new( Vector3::new(10.0, 3.0, 0.0),
                                Vector3::new(-1.0, 0.0, 0.0));
        let mut shaderecord = World::hit_objects( Arc::new(world), &ray, INFINITY);

        assert!(shaderecord.m_ishitting);
        assert_eq!(shaderecord.m_time, 6.0);
        assert_eq!(shaderecord.m_hitpoint, Vector3::new(4.0, 3.0, 0.0));
    }

    #[test]
    fn checkNoHit()
    {
        let mut ray = Ray::new(Vector3::new(7.0, 0.5, 0.0), Vector3::new(-3.0, 3.0, 0.0));
        let mut world = set_up_dummy_world();

        let mut sampler = MultiJittered::new(256, 1);
        let vp = Box::new(ViewPlane::new(Arc::new(sampler)));
        let mut sr = ShadeRec::new(Arc::new(World::new(vp, "whitted")));

        let mut tmin = 100.0;
        let sphere = setUpSphere();
        let res = sphere.hit(&ray, &mut tmin, &mut sr).unwrap();
        assert_eq!(res, false);
    }
}