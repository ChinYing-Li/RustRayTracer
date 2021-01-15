use cgmath::{Vector3, Zero};
use cgmath::num_traits::zero;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::f32::INFINITY;
use std::fs::File;
use std::io::BufReader;
use obj::{Obj};
use std::path::Path;
use std::env;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

use raytracer::utils::color::Colorf;
use raytracer::tracer::whitted::Whitted;
use raytracer::world::{shaderec::ShadeRec,
                       viewplane::ViewPlane,
                       world::World};

use raytracer::ray::Ray;
use raytracer::output::{imagewriter::ImageWriter, OutputManager};

use raytracer::camera::{Camera,
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

use raytracer::geometry::triangle::Triangle;

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
    let args: Vec<String> = env::args().collect();
    // let obj_file_path = &args[1];

    let tracer = Whitted::new();

    let mut boxed_vp = Box::new(ViewPlane::new(Arc::new(
        MultiJittered::new(32, 3))));
    let vp_hres = 800;
    let vp_vres = 600;
    boxed_vp.m_hres = vp_hres;
    boxed_vp.m_vres = vp_vres;
    boxed_vp.m_pixsize = 0.5;
    boxed_vp.set_gamma(1.8);
    boxed_vp.m_maxdepth = 3;

    let mut imgwriter = ImageWriter::new("5_kdtree.jpg", vp_hres, vp_vres);
    let mut world = World::new(boxed_vp, "whitted");

    let objdata = Obj::load("/home/chin-ying/Pictures/bunny.obj")
        .unwrap_or_else(|err| panic!(err)).data;
    let mesh = TriMesh::new(&objdata);
    let mesh_bbox = mesh.m_bbox.clone();
    let mut kdtree = KDTree::<MeshTriangle>::new(
        create_meshtriangles(Arc::new(mesh), &objdata),
                                                 20.0,
                                                 10.0,
                                                 10.0,
                                                 3,
                                                 0);
    kdtree.m_bounds = mesh_bbox;
    kdtree.init();

    let kdtree_ptr = Arc::new(Mutex::new(kdtree));
    world.add_object(kdtree_ptr);

    let c = vec![Colorf::new(0.7, 1.0, 0.0)];
    let objlen= world.m_objects.len();
    let materials: Vec<Arc<dyn Material>> = (0..objlen)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| setUpMaterial(c[*x].m_r, c[*x].m_g, c[*x].m_b, "matte"))
        .collect::<Vec<Arc<dyn Material>>>();

    for i in 0..objlen
    {
        let mut obj = world.m_objects[i].lock().unwrap();
        obj.set_material(materials[i].clone());
    }

    set_up_lights(&mut world);
    let mut ph = setUpCamera();
    ph.m_distance_from_vp = 100.0;
    ph.m_zoom = 1.0;
    ph.m_core.m_exposure_time = 0.05;
    let worldptr = Arc::new(world);
    ph.render_scene(worldptr, &tracer, &mut imgwriter,1.0);
    imgwriter.output();
}

fn set_up_lights(world: &mut World)
{
    let point = PointLight::new(0.2, COLOR_WHITE, Vector3::new(-30.0, 20.0, -20.0));
    let point1 = PointLight::new(0.5, COLOR_RED, Vector3::new(30.0, 10.0, -5.0));
    let point2 = PointLight::new(0.6, COLOR_YELLOW, Vector3::new(70.0, 40.0, 5.0));
    let mut ambient = Ambient::new(COLOR_WHITE);
    ambient.set_radiance_scaling_factor(0.02);
    world.add_light(Arc::new(point));
    world.add_light(Arc::new(point1));
    world.add_light(Arc::new(point2));
    world.set_ambient(Arc::new(ambient));
}

fn setUpMaterial(r: f32, g: f32, b: f32, material_type: &str) -> Arc<dyn Material>
{
    let color = Colorf::new(r, g, b);
    let random_lambertian = Arc::new(Lambertian::new(0.5*g, color.clone()));
    let glossy = Arc::new(GlossySpecular::new(r, color.clone()));
    let phong = Arc::new(Phong::new(random_lambertian.clone(), random_lambertian.clone(),
                                    glossy.clone()));

    return match material_type
    {
        "matte" => // Matte
            {
                Arc::new(Matte::new(random_lambertian.clone(), random_lambertian.clone()))
            }
        "phong" => // Phong
            {
                phong
            }
        "glossy" => // GlossyReflector
            {
                Arc::new(GlossyReflector::new(phong.clone(), glossy.clone()))
            }
        _ =>
            {
                Arc::new(Matte::new(Arc::new(Lambertian::new(0.5 * g, Colorf::new(r, g, b))),
                                    Arc::new(Lambertian::new(0.3, Colorf::new(0.5 * r, g, 0.5 * b)))))
            }
    }

}

fn setUpCamera() -> Pinhole
{
    let eye = Vector3::new(0.0, -30.0, 30.0);
    let lookat = Vector3::new(20.0, 40.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    Pinhole::new(eye, lookat, up)
}