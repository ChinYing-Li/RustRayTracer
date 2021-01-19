pub mod pinhole;

use cgmath::{Vector3,
             InnerSpace,
             dot,
             ElementWise,
             Zero,
             Vector2};
use std::{f32,
          sync::Arc};
use crate::{world::world::World,
            tracer::Tracer,
            output::OutputManager};

type Point3<T> = Vector3<T>;

/// The fundamental component of struct that implements the "Camera" trait.
pub struct CamStruct
{
    pub m_eye: Point3<f32>,
    pub m_lookat: Point3<f32>,
    pub m_up: Vector3<f32>,
    pub m_u: Vector3<f32>,
    pub m_v: Vector3<f32>,
    pub m_w: Vector3<f32>,
    pub m_computed_uvw: bool,
    pub m_exposure_time: f32,
}

impl CamStruct
{
    /// Create a new Camstruct and specified three fields: m_eye, m_lookat and m_up
    pub fn new(eye: Vector3<f32>, lookat: Vector3<f32>, up: Vector3<f32>) -> CamStruct
    {
        CamStruct
        {
            m_eye: eye,
            m_lookat: lookat,
            m_up: up,
            m_u: Vector3::zero(),
            m_v: Vector3::zero(),
            m_w: Vector3::zero(),
            m_computed_uvw: false,
            m_exposure_time: 0.001
        }
    }

    /// Compute the world denotation of axis of render's local coordinate system
    pub fn compute_uvw(&mut self)
    {
        self.m_w = (self.m_eye - self.m_lookat).normalize();

        // rotate "up" a bit when the looking direction
        // is parallel to the "up" vector
        if dot(self.m_w, self.m_up) == self.m_up.magnitude()
        {
            self.m_up.mul_assign_element_wise(Vector3::new(0.99999, 1.0, 1.0));
        }
        self.m_u = self.m_up.cross(self.m_w).normalize();
        self.m_v = self.m_w.cross(self.m_u).normalize();
    }
}

pub trait Camera
{
    fn get_ray_direction(&self, vp_coords: Vector2<f32>) -> Vector3<f32>;
    fn render_scene<'a>(&mut self, worldptr: Arc<World>, outmgr: &'a mut dyn OutputManager);
    fn set_zoom(&mut self, zoom: f32);
    fn get_zoom(&mut self) -> f32;
}

#[cfg(test)]
mod CamStructTest
{
    use super::*;
    use approx::{assert_relative_eq};

    #[test]
    fn compute_uvw_test()
    {
        let lookat = Vector3::new(3.0,4.0,5.0);
        let eye = Vector3::new(2.0, 7.0, 8.0);
        let up = Vector3::unit_z();
        let mut cs = CamStruct::new(eye, lookat, up);

        cs.compute_uvw();
        assert_relative_eq!(cs.m_w, Vector3::new(-0.22941573387056174, 0.6882472016116852, 0.6882472016116852));
    }
}