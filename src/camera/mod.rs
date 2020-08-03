use cgmath::{Vector3, InnerSpace};
use std::{f32};
use crate::world::world::World;

type Point3<T> = Vector3<T>;

pub struct CamStruct
{
    pub m_eye: Point3<f32>,
    pub m_lookat: Point3<f32>,
    pub m_up: Vector3<f32>,
    pub m_u: Vector3<f32>,
    pub m_v: Vector3<f32>,
    pub m_w: Vector3<f32>,
    pub m_exposuretime: f32,
}

pub trait Camera
{
    fn ComputeUVW(camstruct: &mut CamStruct)
    {
        camstruct.m_w = (camstruct.m_eye - camstruct.m_lookat)
                        .normalize();
        camstruct.m_u = camstruct.m_up.cross(camstruct.m_w).normalize();
        camstruct.m_v = camstruct.m_w.cross(camstruct.m_u);
    }

    fn renderScene(&mut self, worldref: &World);
}

pub mod pinhole;