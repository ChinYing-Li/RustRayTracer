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
use raytracer::output::imagewriter::ImageWriter;
use raytracer::output::OutputManager;

use raytracer::camera::{Camera,
                        pinhole::Pinhole};
use raytracer::brdf::lambertian::Lambertian;
use raytracer::brdf::glossyspec::GlossySpecular;

use raytracer::utils::colorconstant::{COLOR_BLUE,
                                      COLOR_RED,
                                      COLOR_WHITE,
                                      COLOR_YELLOW};
use raytracer::light::{ambient::Ambient,
                       pointlight::PointLight};

use raytracer::material::{glossyreflector::GlossyReflector,
                          Material,
                          matte::Matte,
                          phong::Phong,
                          reflector::Reflective};

use raytracer::geometry::triangle::Triangle;

use raytracer::light::ambientocc::AmbientOccluder;
use raytracer::sampler::{Sampler,
                         mutijittered::MultiJittered};
use raytracer::geometry::{cuboid::Cuboid,
                          kdtree::KDTree,
                          instance::Instance,
                          trimesh::{TriMesh, MeshTriangle, create_meshtriangles},
                          sphere::Sphere};
use raytracer::geometry::Shadable;


fn main()
{
    let tracer = RayCast::new();

    let mut boxed_vp = Box::new(ViewPlane::new(Arc::new(MultiJittered::new(32, 3))));
    let vp_hres = 800;
    let vp_vres = 600;
    boxed_vp.m_hres = vp_hres;
    boxed_vp.m_vres = vp_vres;
    boxed_vp.m_pixsize = 0.5;
    boxed_vp.set_gamma(1.8);

    let mut imgwriter = ImageWriter::new("2_sphere_and_triangle_ambientoccluder.jpg", vp_hres, vp_vres);
    let mut world = World::new(boxed_vp);


    let mut sphereA = Arc::new(Mutex::new(Sphere::new(10.0,
                                                      Vector3::new(-12.0, 20.0, 10.0),
                                                      Colorf::new(1.0, 1.0, 0.0))));
    let mut sphereB = Arc::new(Mutex::new(Sphere::new(15.0,
                                                      Vector3::new(30.0, 10.0, 15.0),
                                                      Colorf::new(1.0, 0.0, 1.0))));
    let mut triangle = Arc::new(Mutex::new(Triangle::new(Vector3::new(-100.0, 0.0, 0.0),
                                                         Vector3::new(100.0, 0.0, 0.0),
                                                         Vector3::new(0.0, 200.0, 0.0))));
    world.add_object(sphereA);
    world.add_object(sphereB);
    world.add_object(triangle);

    let c = vec![Colorf::new(0.7, 1.0, 0.0), Colorf::new(0.6, 0.7, 0.8), Colorf::new(0.0, 1.0, 1.0)];
    let objlen= world.m_objects.len();
    let materials: Vec<Matte> = (0..objlen)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| setUpMaterial(c[*x].m_r, c[*x].m_g, c[*x].m_b))
        .collect::<Vec<Matte>>();

    for i in 0..objlen
    {
        let mut obj = world.m_objects[i].lock().unwrap();
        obj.set_material(Arc::new(materials[i].clone()));
    }

    setUpAmbientOccluder(&mut world);
    let mut ph = setUpCamera();
    ph.m_distance_from_vp = 100.0;
    ph.m_zoom = 1.0;
    ph.m_core.m_exposure_time = 0.005;
    let worldptr = Arc::new(world);
    ph.render_scene(worldptr, &tracer, &mut imgwriter,1.0);
    imgwriter.output();
}

fn setUpMaterial(r: f32, g: f32, b: f32) -> Matte
{
    Matte::new(Arc::new(Lambertian::new(0.5*g, Colorf::new(r, g, b))),
               Arc::new(Lambertian::new(0.3, Colorf::new(0.5*r, g, 0.5*b))))
}

fn setUpAmbientOccluder(world: &mut World)
{
    let mut mj= MultiJittered::new(32, 3);
    mj.set_map_to_hemisphere(true, 1.0);
    mj.generate_sample_pattern();

    let mut ambocc = AmbientOccluder::new(Colorf::new(0.0, 0.1, 0.0), 0.5, Arc::new(mj));
    ambocc.set_color(COLOR_WHITE);
    world.set_ambient(Arc::new(ambocc));
}

fn setUpCamera() -> Pinhole
{
    let eye = Vector3::new(0.0, -30.0, 30.0);
    let lookat = Vector3::new(20.0, 30.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    Pinhole::new(eye, lookat, up)
}