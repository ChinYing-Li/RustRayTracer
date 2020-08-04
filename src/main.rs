use cgmath::Vector3;
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

fn main()
{
    let mut sphere = Sphere::new(30.0,
                                  Vector3::new(70.0, 30.0, 100.0),
                                 Colorf::new(0.0, 1.0, 0.0));
    sphere.setColor(Colorf::new(0.5, 0.7, 0.0));
    let tracer = Box::new(Whitted::new());
    let boxed_vp = Box::new(ViewPlane::new());

    let mut world = World::new(boxed_vp, tracer);
    world.addObject(Arc::new(sphere));

    let mut ray = Ray::new(Vector3::new(0.0, 0.0, 0.0),
                           Vector3::new(0.0, 0.0, 1.0));
    let mut shaderecord = world.hitObjects(&mut ray, INFINITY);
    let filedest = "test.jpg";

    let width = 200;
    let height = 100;
    let mut imgwriter = ImageWriter::new(filedest, width, height);

    for i in 0..width
    {
        for j in 0..height
        {
            ray.m_origin.x = i as f32;
            ray.m_origin.y = j as f32;
            shaderecord = world.hitObjects(&mut ray, INFINITY);
            imgwriter.writePixel(i, j, shaderecord.m_color);
        }
    }

    imgwriter.output();
}
