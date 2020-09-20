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
use raytracer::utils::colorconstant::{COLOR_BLUE, COLOR_RED, COLOR_WHITE, COLOR_YELLOW};
use raytracer::light::pointlight::PointLight;
use raytracer::material::Material;
use std::rc::Rc;
use raytracer::light::ambient::Ambient;
use raytracer::geometry::triangle::Triangle;
use raytracer::material::matte::Matte;
use raytracer::light::ambientocc::AmbientOccluder;
use raytracer::sampler::mutijittered::MultiJittered;
use raytracer::sampler::Sampler;
use raytracer::geometry::cuboid::Cuboid;
use raytracer::geometry::instance::Instance;
use obj::Obj;
use raytracer::geometry::trimesh::{TriMesh, MeshTriangle};
use raytracer::geometry::kdtree::KDTree;

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

    let mut imgwriter = ImageWriter::new("3_reflective_spheres.jpg", vp_hres, vp_vres);
    let mut world = World::new(boxed_vp);

    let mut sphereA = Arc::new(Mutex::new(Sphere::new(10.0,
                                                      Vector3::new(12.0, 20.0, 15.0),
                                                      Colorf::new(0.0, 1.0, 0.0))));
    let mut cuboid = Arc::new(Mutex::new(Cuboid::new(Vector3::new(30.0, 20.0, 20.0),
                                                      Vector3::new(50.0, 40.0, 50.),
                                                      Colorf::new(1.0, 0.0, 0.0))));
    let mut triangle = Arc::new(Mutex::new(Triangle::new(Vector3::new(-10.0, 40.0, 10.0),
                                                         Vector3::new(30.0, 40.0, 0.0),
                                                         Vector3::new(60.0, 40.0, 30.0))));

    world.add_object(sphereA);
    world.add_object(cuboid);
    world.add_object(triangle);

    let dragon_color = Matte::new(Arc::new(Lambertian::new(0.5, Colorf::new(1.0, 0.0, 0.0))),
               Arc::new(Lambertian::new(0.3, Colorf::new(0.5, 0.0, 0.5))));
    let dragon = create_from_obj("", Arc::new(dragon_color));

    world.add_object(Arc::new(Mutex::new(dragon)));
    /*
    for i in 0..5
    {
        let mut instance = Instance::new(sphereA.clone());
        instance.translate(Vector3::new(i as f32 *10.0, i as f32 * 5.0, i as f32 * 5.0));
        world.add_object(Arc::new(Mutex::new(instance)));
    }*/

    let materials: Vec<Matte> = (0..3).collect::<Vec<_>>().iter()
        .map(|x| setUpMaterial(1.0/(*x) as f32, 0.3 * (*x) as f32, 0.5))
        .collect::<Vec<Matte>>();

    for i in 0..3
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

fn setUpMaterial(r: f32, g: f32, b: f32) -> Matte
{
    Matte::new(Arc::new(Lambertian::new(0.5*g, Colorf::new(r, g, b))),
               Arc::new(Lambertian::new(0.3, Colorf::new(0.5*r, g, 0.5*b))))
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
    let eye = Vector3::new(20.0, 30.0, -10.0);
    let lookat = Vector3::new(20.0, 30.0, 100.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    Pinhole::new(eye, lookat, up)
}

fn create_from_obj(path_to_obj: &str, material_ptr: Arc<dyn Material>) -> KDTree<MeshTriangle>
{
    let objdata = Obj::load(path_to_obj)
        .unwrap_or(panic!("The path is not valid; can't load .obj file")).data;
    let mesh = TriMesh::new(&objdata, material_ptr.clone());
    let mut kdtree = KDTree::<MeshTriangle>::new(&mesh.create_triangles(&objdata),
                                             20.0,
                                             10.0,
                                             10.0,
                                             3,
                                             -1);
    kdtree.init();
    kdtree
}