use cgmath::{Vector3, InnerSpace, dot, ElementWise, Zero};
use std::{f32};
use crate::world::world::World;
use crate::camera::pinhole::Pinhole;

type Point3<T> = Vector3<T>;

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
            m_exposure_time: 5.0
        }
    }

    pub fn ComputeUVW(&mut self)
    {
        self.m_w = (self.m_eye - self.m_lookat)
            .normalize();

        // rotate "up" a bit when the looking direction
        // is parallel to the "up" vector
        if dot(self.m_w, self.m_up) == self.m_up.magnitude()
        {
            self.m_up.mul_assign_element_wise(Vector3::new(0.99999, 1.0, 1.0));
        }
        self.m_u = self.m_up.cross(self.m_w).normalize();
        self.m_v = self.m_w.cross(self.m_u);
    }
}

pub trait Camera
{
    fn renderScene(&mut self, worldref: &World);
}

pub mod pinhole;

#[cfg(test)]
mod CamStructTest
{
    use super::*;

    use approx::{assert_relative_eq};

    #[test]
    fn ComputeUVWTest()
    {
        let lookat = Vector3::new(3.0,4.0,5.0);
        let eye = Vector3::new(2.0, 7.0, 8.0);
        let up = Vector3::unit_z();
        let mut cs = CamStruct::new(eye, lookat, up);

        cs.ComputeUVW();
        assert_relative_eq!(cs.m_w, Vector3::new(-0.22941573387056174, 0.6882472016116852, 0.6882472016116852));
    }
}