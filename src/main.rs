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

fn main()
{
    let mut sphereA = Sphere::new(30.0,
                                  Vector3::new(70.0, 30.0, 20.0),
                                 Colorf::new(0.0, 1.0, 0.0));
    sphereA.setColor(Colorf::new(0.5, 0.7, 0.0));
    let mut sphereB = Sphere::new(30.0,
                                    Vector3::new(80.0, 90.0, 100.0),
                                    Colorf::new(1.0, 1.0, 0.0));

    let tracer = Box::new(Whitted::new());

    let mut boxed_vp = Box::new(ViewPlane::new());
    boxed_vp.m_pixsize = 0.5;
    boxed_vp.m_numsample = 3;

    let mut imgwriter = Box::new(ImageWriter::new("test.jpg", 100, 100));
    let mut world = World::new(boxed_vp, tracer, imgwriter);

    world.addObject(Arc::new(sphereA));
    world.addObject(Arc::new(sphereB));

    let eye = Vector3::new(10.0, 20.0, -10.0);
    let lookat = Vector3::new(20.0, 30.0, 100.0);
    let up = Vector3::new(0.0, 1.0, 0.0);

    let ph_camera = Pinhole::new(eye, lookat, up);

    let width = 200;
    let height = 100;
    let mut r = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    let mut shaderecord = world.hitObjects(&mut r, INFINITY);
    for i in 0..width
    {
        for j in 0..height
        {
            r.m_origin.x = i as f32;
            r.m_origin.y = j as f32;
            shaderecord = world.hitObjects(&mut r, INFINITY);
            world.writePixel(i, j, shaderecord.m_color);
        }
    }
    world.output();
}
