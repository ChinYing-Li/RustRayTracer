use cgmath::{Vector3, Zero};
use std::sync::{Arc, Mutex};
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
use raytracer::utils::colorconstant::{COLOR_BLUE, COLOR_RED, COLOR_WHITE};
use raytracer::light::pointlight::PointLight;
use raytracer::material::Material;
use std::rc::Rc;
use raytracer::light::ambient::Ambient;
use raytracer::geometry::triangle::Triangle;
use raytracer::material::matte::Matte;

fn main()
{
    let tracer = Whitted::new();

    let mut boxed_vp = Box::new(ViewPlane::new());
    let vp_hres = 800;
    let vp_vres = 600;
    boxed_vp.m_hres = vp_hres;
    boxed_vp.m_vres = vp_vres;
    boxed_vp.m_pixsize = 0.5;
    boxed_vp.m_numsample = 32;

    let mut imgwriter = ImageWriter::new("2_sphere_and_triangle.jpg", vp_hres, vp_vres);
    let mut world = World::new(boxed_vp);


    let mut sphereA = Arc::new(Mutex::new(Sphere::new(10.0,
                                                      Vector3::new(12.0, 20.0, 15.0),
                                                      Colorf::new(0.0, 1.0, 0.0))));
    let mut sphereB = Arc::new(Mutex::new(Sphere::new(15.0,
                                                      Vector3::new(30.0, 10.0, 20.),
                                                      Colorf::new(1.0, 0.0, 0.0))));
    let mut triangle = Arc::new(Mutex::new(Triangle::new(Vector3::new(-10.0, 10.0, 10.0),
                                                         Vector3::new(30.0, 50.0, 0.0),
                                                         Vector3::new(60.0, 50.0, 30.0))));
    world.add_object(sphereA);
    world.add_object(sphereB);
    world.add_object(triangle);

    let c = vec![COLOR_BLUE, COLOR_RED, Colorf::new(0.0, 1.0, 1.0)];
    let objlen= world.m_objects.len();
    let materials: Vec<Phong> = (0..objlen)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| setUpMaterial(1.0/(*x) as f32, 0.3 * (*x) as f32, 0.5))
        .collect::<Vec<Phong>>();

    for i in 0..objlen
    {
        let mut obj = world.m_objects[i].lock().unwrap();
        obj.set_material(Arc::new(materials[i].clone()));
    }

    setUpLights(&mut world);
    let mut ph = setUpCamera();
    ph.m_distance_from_vp = 100.0;
    ph.m_zoom = 1.0;
    ph.m_core.m_exposure_time = 0.05;
    let worldptr = Arc::new(world);
    ph.render_scene(worldptr, &tracer, &mut imgwriter,1.0);
    imgwriter.output();
}

fn setUpObjects(world: &mut World)
{

}

fn setUpMaterial(r: f32, g: f32, b: f32) -> Phong
{
    Phong::new(Arc::new(Lambertian::new(0.5*g, Colorf::new(r, g, b))),
               Arc::new(Lambertian::new(0.3, Colorf::new(0.5*r, g, 0.5*b))),
               Arc::new(GlossySpecular::new(0.4*r, Colorf::new(0.3*r, g, b))))
}

fn setUpLights(world: &mut World)
{
    let point = PointLight::new(0.5, COLOR_WHITE, Vector3::new(-30.0, 20.0, -20.0));
    let point2 = PointLight::new(0.5, COLOR_RED, Vector3::new(30.0, 10.0, -5.0));
    let mut ambient = Ambient::new(COLOR_BLUE);
    ambient.set_radiance_scaling_factor(0.1);
    world.add_light(Arc::new(point));
    world.add_light(Arc::new(point2));
    world.set_ambient(Arc::new(ambient));
}

fn setUpCamera() -> Pinhole
{
    let eye = Vector3::new(20.0, 30.0, -10.0);
    let lookat = Vector3::new(20.0, 30.0, 100.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    Pinhole::new(eye, lookat, up)
}