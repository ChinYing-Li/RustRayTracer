use cgmath::{Vector3, Zero};
use std::sync::Arc;
use std::f32::INFINITY;

use raytracer::utils::color::Colorf;
use raytracer::tracer::whitted::Whitted;
use raytracer::world::viewplane::ViewPlane;
use raytracer::world::world::World;

use raytracer::geometry::sphere::Sphere;
use raytracer::ray::Ray;
use raytracer::utils::shaderec::ShadeRec;
use raytracer::output::imagewriter::ImageWriter;
use raytracer::output::OutputManager;
use raytracer::camera::pinhole::Pinhole;
use cgmath::num_traits::zero;
use raytracer::camera::Camera;
use raytracer::material::phong::Phong;
use raytracer::brdf::lambertian::Lambertian;
use raytracer::brdf::glossyspec::GlossySpecular;
use raytracer::utils::colorconstant::{COLOR_BLUE, COLOR_RED};
use raytracer::light::pointlight::PointLight;
use raytracer::material::Material;
use std::rc::Rc;

fn main()
{
    let tracer = Whitted::new();

    let mut boxed_vp = Box::new(ViewPlane::new());
    let vp_hres = 100;
    let vp_vres = 200;
    boxed_vp.m_hres = vp_hres;
    boxed_vp.m_vres = vp_vres;
    boxed_vp.m_pixsize = 0.5;
    boxed_vp.m_numsample = 3;

    let mut imgwriter = ImageWriter::new("test.jpg", vp_hres, vp_vres);
    let mut world = World::new(boxed_vp);


    let mut sphereA = Arc::new(Sphere::new(30.0,
                                           Vector3::new(70.0, 30.0, 20.0),
                                           Colorf::new(0.0, 1.0, 0.0)));
    let mut sphereB = Arc::new(Sphere::new(30.0,
                                           Vector3::new(80.0, 90.0, 100.0),
                                           Colorf::new(1.0, 1.0, 0.0)));
    world.addObject(sphereA);
    world.addObject(sphereB);

    let c = vec![COLOR_BLUE, COLOR_RED];
    let objlen= world.m_objects.len();
    let materials: Vec<Phong> = (0..objlen)
                                            .collect::<Vec<_>>()
                                            .iter()
                                            .map(|x| setUpMaterial(1.0/(*x) as f32, 0.3 * (*x) as f32, 0.5))
                                            .collect::<Vec<Phong>>();

    for i in 0..objlen
    {
        world.m_objects[i].setMaterial(&(materials[i]));
    }
    setUpLights(&mut world);
    let mut ph = setUpCamera();
    let worldptr = Rc::new(world);
    ph.renderScene(worldptr, &tracer, &mut imgwriter,1.0);
    imgwriter.output();
}

fn setUpObjects(world: &mut World)
{

}

fn setUpMaterial(r: f32, g: f32, b: f32) -> Phong
{
    Phong::new(Arc::new(Lambertian::new(0.5, Colorf::new(r, g, b))),
                                                                Arc::new(Lambertian::new(0.8, Colorf::new(0.5*r, g, 0.5*b))),
                                                                    Arc::new(GlossySpecular::new(1.0, Colorf::new(r, g, b))))
}

fn setUpLights(world: &mut World)
{
    let point = PointLight::new(2.0, COLOR_RED, Vector3::new(50.0, 60.0, 50.0));
    world.addLight(Arc::new(point));
}

fn setUpCamera() -> Pinhole
{
    let eye = Vector3::new(10.0, 20.0, -10.0);
    let lookat = Vector3::new(20.0, 30.0, 100.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    Pinhole::new(eye, lookat, up)
}