use cgmath::{Vector3, Vector2, Zero, ElementWise, InnerSpace};
use std::sync::Arc;

use crate::{render::cam::{CamStruct, Camera},
            ray::Ray,
            world::world::World};
use crate::utils::colorconstant::COLOR_BLACK;
use crate::output::OutputManager;
use cgmath::num_traits::Inv;
use crate::render::renderbuffer::RenderBuffer;
use crate::render::renderdata::RenderMeta;
use crate::utils::color::Colorf;


pub struct Pinhole
{
    m_zoom: f32,
    m_inv_zoom: f32,
    pub m_core: CamStruct,
    pub m_distance_from_vp: f32,
}

impl Pinhole
{
    pub fn new(eye: Vector3<f32>, lookat: Vector3<f32>, up: Vector3<f32>) -> Pinhole
    {
        let mut core = CamStruct::new(eye, lookat, up);
        core.compute_uvw();
        Pinhole{ m_core: core, m_zoom: 1.0, m_inv_zoom: 1.0, m_distance_from_vp: 50.0}
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
    // Should render to buffer...
    fn render_scene<'a>(&mut self, worldptr: Arc<World>, outmgr: &'a mut dyn OutputManager)
    {
        let mut clr = COLOR_BLACK;
        let vp = worldptr.m_viewplaneptr.as_ref();
        let mut ray = Ray::new(self.m_core.m_eye, Vector3::new(0.0, 0.0, 1.0));
        let mut actual_sample_point = Vector2::zero();

        for x in 0..vp.m_hres
        {
            for y in 0..vp.m_vres
            {
                clr = COLOR_BLACK;

                for sample in vp.m_sampler.get_unit_square_pattern()
                {
                    // TODO: Shall we make sampler a member of viewplane ?
                    actual_sample_point = vp.get_coordinate_from_index(x, y)
                                            .unwrap_or(Vector2::zero())
                                            .add_element_wise(*sample);
                    ray.m_direction = self.get_ray_direction(actual_sample_point);
                    clr += worldptr.as_ref().m_tracer.trace_ray(worldptr.clone(), &ray, 0);
                    // TODO: Why should Tracer be part of the World class
                }
                let r_before = clr.m_g;
                // print!("r_before {}", r_before.to_string());
                clr /= (vp.m_sampler.get_sample_per_pattern() as f32);
                // print!("r_after {}", clr.m_g.to_string());
                clr *= self.m_core.m_exposure_time;
                clr.clamp();
                outmgr.write_pixel(x.into(), y.into(), clr, vp.get_inv_gamma());
            }
        }
    }

    fn render(&self, world_ptr: Arc<World>, rendermeta: &RenderMeta) -> Vec<Colorf>
    {
        let mut clr = COLOR_BLACK;
        let vp = world_ptr.m_viewplaneptr.as_ref();
        let mut ray = Ray::new(self.m_core.m_eye, Vector3::new(0.0, 0.0, 1.0));
        let mut actual_sample_point = Vector2::zero();
        let start_coords = rendermeta.get_start_coords();
        let end_coords = rendermeta.get_end_coords();
        let mut samples = Vec::with_capacity(rendermeta.m_area);

        for x in start_coords.0..end_coords.0
        {
            for y in start_coords.1..end_coords.1
            {
                clr = COLOR_BLACK;

                for sample in vp.m_sampler.get_unit_square_pattern()
                {
                    // TODO: Shall we make sampler a member of viewplane ?
                    actual_sample_point = vp.get_coordinate_from_index(x as u16, y as u16)
                        .unwrap_or(Vector2::zero())
                        .add_element_wise(*sample);
                    ray.m_direction = self.get_ray_direction(actual_sample_point);
                    clr += world_ptr.as_ref().m_tracer.trace_ray(world_ptr.clone(), &ray, 0);
                    // TODO: Why should Tracer be part of the World class
                }
                let r_before = clr.m_g;
                // print!("r_before {}", r_before.to_string());
                clr /= (vp.m_sampler.get_sample_per_pattern() as f32); // this seems quite fishy
                // print!("r_after {}", clr.m_g.to_string());
                clr *= self.m_core.m_exposure_time;
                clr.clamp();
                samples.push(clr);
            }
        }
        samples
    }

    fn set_zoom(&mut self, zoom: f32)
    {
        if zoom == 0.0
        {
            panic!("Zoom factor can't be 0")
        }
        else
        {
            self.m_zoom = zoom;
            self.m_inv_zoom = zoom.inv();
        }
    }

    fn get_zoom(&mut self) -> f32
    {
        self.m_zoom
    }
}