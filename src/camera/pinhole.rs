use crate::{camera::{CamStruct, Camera},
            ray::Ray,
            world::world::World,
            utils::color::Colorf};

use cgmath::{Vector3, Vector2, Zero, ElementWise, InnerSpace};
use rand::Rng;
use crate::utils::colorconstant::COLOR_BLACK;

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
        Pinhole{ m_core: core, m_zoom: 1.0, m_distance_from_vp: 5.0}
    }
}

impl Camera for Pinhole
{
    fn getRayDirection(&self, vp_coords: Vector2<f32>) -> Vector3<f32>
    {
         (self.m_core.m_u.mul_element_wise(vp_coords.x)
             + self.m_core.m_v.mul_element_wise(vp_coords.y)
             + self.m_core.m_w.mul_element_wise(self.m_distance_from_vp))
             .normalize()
    }

    fn renderScene(&mut self, worldref: &mut World, zoom: f32)
    {
        let mut clr = COLOR_BLACK;
        let mut vp = (worldref.m_viewplaneptr).clone();
        let mut ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let mut sq_sample_point = 0.0;
        let mut actual_sample_point = Vector2::zero();
        let mut rng = rand::thread_rng();

        vp.m_pixsize /= zoom;

        for x in 0..vp.m_hres
        {
            for y in 0..vp.m_vres
            {
                clr = COLOR_BLACK;

                for i in 0..vp.m_numsample
                {
                    sq_sample_point = rng.gen_range(0.0, 1.0);
                    actual_sample_point = vp.getCoordinateFromIndex(x, y)
                                            .unwrap()
                                            .add_element_wise(sq_sample_point);
                    ray.m_velocity = self.getRayDirection(actual_sample_point);
                    clr += worldref.m_tracerptr.traceRay(worldref, &ray, 0);
                }
                clr /= vp.m_numsample as f32;
                clr *= self.m_core.m_exposure_time;
                worldref.writePixel(x.into(), y.into(), clr);
            }
        }
    }
}