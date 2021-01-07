use cgmath::{Vector3, Vector2, Zero, ElementWise, InnerSpace};
use rand::Rng;
use std::sync::Arc;

use crate::{camera::{CamStruct, Camera},
            ray::Ray,
            world::world::World};
use crate::utils::colorconstant::COLOR_BLACK;
use crate::tracer::Tracer;
use crate::output::OutputManager;


pub struct Pinhole
{
    pub m_core: CamStruct,
    pub m_zoom: f32,
    pub m_distance_from_vp: f32,
}

impl Pinhole
{
    pub fn new(eye: Vector3<f32>, lookat: Vector3<f32>, up: Vector3<f32>) -> Pinhole
    {
        let mut core = CamStruct::new(eye, lookat, up);
        core.ComputeUVW();
        Pinhole{ m_core: core, m_zoom: 1.0, m_distance_from_vp: 50.0}
    }
}

impl Camera for Pinhole
{
    fn get_ray_direction(&self, vp_coords: Vector2<f32>) -> Vector3<f32>
    {
         (self.m_core.m_u.mul_element_wise(vp_coords.x)
             + self.m_core.m_v.mul_element_wise(vp_coords.y)
             - self.m_core.m_w.mul_element_wise(self.m_distance_from_vp))
             .normalize()
    }

    fn render_scene<'a>(&mut self, worldptr: Arc<World>, tracer: &'a dyn Tracer, outmgr: &'a mut dyn OutputManager, zoom: f32)
    {
        let mut clr = COLOR_BLACK;
        let mut vp = (worldptr.m_viewplaneptr).clone();
        let mut ray = Ray::new(self.m_core.m_eye, Vector3::new(0.0, 0.0, 1.0));
        let mut sq_sample_point = 0.0;
        let mut actual_sample_point = Vector2::zero();
        let mut rng = rand::thread_rng();

        vp.m_pixsize /= zoom;

        for x in 0..vp.m_hres
        {
            for y in 0..vp.m_vres
            {
                clr = COLOR_BLACK;

                for _i in 0..vp.m_sampler.get_sample_per_pattern()
                {
                    // TODO use Sampler !!!
                    sq_sample_point = rng.gen_range(0.0, 1.0);
                    actual_sample_point = vp.get_coordinate_from_index(x, y)
                                            .unwrap_or(Vector2::zero())
                                            .add_element_wise(sq_sample_point);

                    ray.m_direction = self.get_ray_direction(actual_sample_point);
                    clr += tracer.trace_ray(worldptr.clone(), &ray, 0);
                }
                clr /= vp.m_sampler.get_sample_per_pattern() as f32;
                clr *= self.m_core.m_exposure_time;
                outmgr.write_pixel(x.into(), y.into(), clr, vp.get_inv_gamma());
            }
        }
    }
}