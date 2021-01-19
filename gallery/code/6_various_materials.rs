use cgmath::{Vector3, Zero};
use cgmath::num_traits::zero;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::f32::INFINITY;
use std::fs::File;
use std::io::BufReader;
use obj::{Obj};

use raytracer::utils::color::Colorf;
use raytracer::tracer::whitted::Whitted;
use raytracer::world::viewplane::ViewPlane;
use raytracer::world::world::World;

use raytracer::geometry::sphere::Sphere;
use raytracer::ray::Ray;
use raytracer::world::shaderec::ShadeRec;
use raytracer::output::imagewriter::ImageWriter;
use raytracer::output::OutputManager;
use raytracer::camera::pinhole::Pinhole;
use raytracer::camera::Camera;
use raytracer::material::phong::Phong;
use raytracer::material::glossyreflector::GlossyReflector;
use raytracer::brdf::lambertian::Lambertian;
use raytracer::brdf::glossyspec::GlossySpecular;
use raytracer::utils::colorconstant::{COLOR_BLUE, COLOR_RED, COLOR_WHITE, COLOR_YELLOW};
use raytracer::light::pointlight::PointLight;
use raytracer::material::Material;
use raytracer::light::ambient::Ambient;
use raytracer::geometry::triangle::Triangle;
use raytracer::material::matte::Matte;
use raytracer::light::ambientocc::AmbientOccluder;
use raytracer::sampler::mutijittered::MultiJittered;
use raytracer::sampler::Sampler;
use raytracer::geometry::cuboid::Cuboid;
use raytracer::geometry::instance::Instance;
use raytracer::geometry::trimesh::{TriMesh, MeshTriangle, create_meshtriangles};
use raytracer::geometry::kdtree::KDTree;
use raytracer::geometry::Shadable;
use std::path::Path;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

fn main()
{
    let tracer = Whitted::new();

    let mut boxed_vp = Box::new(ViewPlane::new(Arc::new(MultiJittered::new(32, 3))));
    let vp_hres = 800;
    let vp_vres = 600;
    boxed_vp.m_hres = vp_hres;
    boxed_vp.m_vres = vp_vres;
    boxed_vp.m_pixsize = 0.5;
    boxed_vp.set_gamma(1.8);

    let mut imgwriter = ImageWriter::new("4_bunny.jpg", vp_hres, vp_vres);
    let mut world = World::new(boxed_vp, "whitted");

    let mut sphere = Arc::new(Mutex::new(Sphere::new(10.0,
                                                     Vector3::new(0.0, 0.0, 5.0),
                                                     Colorf::new(0.0, 1.0, 0.0))));
    let mut cuboid = Arc::new(Mutex::new(Cuboid::new(Vector3::new(0.0, 0.0, 0.0),
                                                     Vector3::new(10.0, 10.0, 10.),
                                                     Colorf::new(1.0, 0.0, 0.0))));

    let mut rng = thread_rng();

    for i in 0..5
    {
        for j in 0..5
        {
            sphere.lock().unwrap().m_center = Vector3::new(i as f32 * 5.0 + rng.gen_range(-3.0, 3.0),
                                                            j as f32 * 5.0 + rng.gen_range(-3.0, 3.0),
                                                            i as f32 * rng.gen_range(-10.0, 10.0) + j as f32 * rng.gen_range(-10.0, 10.0));
            world.add_object(sphere.clone());

            let displacement = Vector3::new(i as f32 * 15.0 + rng.gen_range(-3.0, 3.0),
                                            j as f32 * 15.0 + rng.gen_range(-3.0, 3.0),
                                            i as f32 * rng.gen_range(-10.0, 10.0) + j as f32 * rng.gen_range(-10.0, 10.0));
            cuboid.lock().unwrap().m_vec0 = displacement;
            cuboid.lock().unwrap().m_vec1 = displacement;

            world.add_object(cuboid.clone());
        }
    }



    let mut triangle = Arc::new(Mutex::new(Triangle::new(Vector3::new(-10.0, 40.0, 10.0),
                                                         Vector3::new(30.0, 40.0, 0.0),
                                                         Vector3::new(60.0, 40.0, 1000.0))));

    world.add_object(triangle);

    let matte_materials: Vec<Arc<dyn Material>> = (0..4).collect::<Vec<_>>().iter()
        .map(|x| setUpMaterial(rng.gen_range(0.0, 1.0) * (*x) as f32,
                               rng.gen_range(0.0, 1.0) * (*x) as f32,
                               rng.gen_range(0.0, 1.0) * (*x) as f32,
                               "matte"))
        .collect::<Vec<Arc<dyn Material>>>();

    let phong_materials: Vec<Arc<dyn Material>> = (0..4).collect::<Vec<_>>().iter()
        .map(|x| setUpMaterial(rng.gen_range(0.0, 1.0)* (*x) as f32,
                               rng.gen_range(0.0, 1.0) * (*x) as f32,
                               rng.gen_range(0.0, 1.0)* (*x) as f32,
                               "phong"))
        .collect::<Vec<Arc<dyn Material>>>();

    let glossy_materials: Vec<Arc<dyn Material>> = (0..4).collect::<Vec<_>>().iter()
        .map(|x| setUpMaterial(rng.gen_range(0.0, 1.0)* (*x) as f32,
                               rng.gen_range(0.0, 1.0) * (*x) as f32,
                               rng.gen_range(0.0, 1.0)* (*x) as f32,
                               "glossy"))
        .collect::<Vec<Arc<dyn Material>>>();

    let mut rand_uint = 0 as u8;
    for i in 0..4
    {
        rand_uint = thread_rng().gen();
        let mut obj = world.m_objects[i].lock().unwrap();
        let index = (rand_uint % 4) as usize;
        match rand_uint % 3
        {
            1 => obj.set_material(matte_materials[index].clone()),
            2 => obj.set_material(phong_materials[index].clone()),
            _ => obj.set_material(glossy_materials[index].clone()),
        }
    }

    setUpLights(&mut world);
    let mut ph = setUpCamera();
    ph.m_distance_from_vp = 100.0;
    ph.m_zoom = 1.0;
    ph.m_core.m_exposure_time = 0.05;
    let worldptr = Arc::new(world);
    ph.render_scene(worldptr, &mut imgwriter,1.0);
    imgwriter.output();
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

fn setUpLights(world: &mut World)
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

fn setUpAmbientOccluder(world: &mut World)
{
    let mut mj= MultiJittered::new(32, 3);
    mj.set_map_to_hemisphere(true, 1.0);
    mj.generate_sample_pattern();

    let mut ambocc = AmbientOccluder::new(0.0, 0.3, Arc::new(mj));
    ambocc.set_color(COLOR_BLUE);
    world.add_light(Arc::new(ambocc));

    let mut ambient = Ambient::new(COLOR_WHITE);
    ambient.set_radiance_scaling_factor(0.1);
    world.set_ambient(Arc::new(ambient));
}

fn setUpCamera() -> Pinhole
{
    let eye = Vector3::new(20.0, 30.0, -100.0);
    let lookat = Vector3::new(20.0, 30.0, 100.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    Pinhole::new(eye, lookat, up)
}