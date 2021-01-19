use cgmath::{Vector3, Zero};
use cgmath::num_traits::zero;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::f32::INFINITY;
use std::fs::File;
use std::io::BufReader;
use obj::{Obj};
use std::path::Path;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use raytracer::utils::color::Colorf;
use raytracer::tracer::whitted::Whitted;
use raytracer::world::{shaderec::ShadeRec,
                       viewplane::ViewPlane,
                       world::World};

use raytracer::ray::Ray;
use raytracer::output::{imagewriter::ImageWriter, OutputManager};
use raytracer::render::cam::{Camera,
                            pinhole::Pinhole};
use raytracer::brdf::lambertian::Lambertian;
use raytracer::brdf::glossyspec::GlossySpecular;

use raytracer::utils::colorconstant::{COLOR_BLUE,
                                      COLOR_RED,
                                      COLOR_WHITE,
                                      COLOR_YELLOW};
use raytracer::light::{pointlight::PointLight,
                       ambient::Ambient,
                       ambientocc::AmbientOccluder};
use raytracer::material::{glossyreflector::GlossyReflector,
                          Material,
                          matte::Matte,
                          phong::Phong,
                          reflector::Reflective};
use raytracer::sampler::{Sampler,
                         mutijittered::MultiJittered};
use raytracer::geometry::{cuboid::Cuboid,
                          kdtree::KDTree,
                          instance::Instance,
                          triangle::Triangle,
                          trimesh::{TriMesh, MeshTriangle, create_meshtriangles},
                          Shadable,
                          sphere::Sphere};

fn main()
{
    let mut boxed_vp = Box::new(ViewPlane::new(Arc::new(
        MultiJittered::new(32, 3))));
    let vp_hres = 800;
    let vp_vres = 600;
    let zoom = 1.0_f32;
    boxed_vp.m_hres = vp_hres;
    boxed_vp.m_vres = vp_vres;
    boxed_vp.m_pixsize = 0.5;
    boxed_vp.set_zoom(&zoom);

    let mut imgwriter = ImageWriter::new("2_sphere_and_triangle.jpg", vp_hres, vp_vres);
    let mut world = World::new(boxed_vp, "whitted");

    let mut sphereA = Arc::new(Mutex::new(Sphere::new(10.0,
                                                      Vector3::new(12.0, 20.0, 15.0))));
    let mut sphereB = Arc::new(Mutex::new(Sphere::new(15.0,
                                                      Vector3::new(30.0, 10.0, 20.))));
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
        .map(|x| set_up_material(1.0/(*x) as f32, 0.3 * (*x) as f32, 0.5))
        .collect::<Vec<Phong>>();

    for i in 0..objlen
    {
        let mut obj = world.m_objects[i].lock().unwrap();
        obj.set_material(Arc::new(materials[i].clone()));
    }

    set_up_lights(&mut world);
    let mut ph = set_up_camera();
    ph.m_distance_from_vp = 100.0;
    ph.set_zoom(zoom);
    ph.m_core.m_exposure_time = 0.05;
    let worldptr = Arc::new(world);
    ph.render_scene(worldptr, &mut imgwriter,1.0);
    imgwriter.output();
}

fn set_up_material(r: f32, g: f32, b: f32) -> Phong
{
    Phong::new(Arc::new(Lambertian::new(0.5*g, Colorf::new(r, g, b))),
               Arc::new(Lambertian::new(0.3, Colorf::new(0.5*r, g, 0.5*b))),
               Arc::new(GlossySpecular::new(0.4*r, Colorf::new(0.3*r, g, b))))
}

fn set_up_lights(world: &mut World)
{
    let point = PointLight::new(0.5, COLOR_WHITE, Vector3::new(-30.0, 20.0, -20.0));
    let point2 = PointLight::new(0.5, COLOR_RED, Vector3::new(30.0, 10.0, -5.0));
    let mut ambient = Ambient::new(COLOR_BLUE);
    ambient.set_radiance_scaling_factor(0.1);
    world.add_light(Arc::new(point));
    world.add_light(Arc::new(point2));
    world.set_ambient(Arc::new(ambient));
}

fn set_up_camera() -> Pinhole
{
    let eye = Vector3::new(20.0, 30.0, -10.0);
    let lookat = Vector3::new(20.0, 30.0, 100.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    Pinhole::new(eye, lookat, up)
}