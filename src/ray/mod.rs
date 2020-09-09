use std::f32;
use cgmath::prelude::*;
use cgmath::Vector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray
{
    pub m_origin: Vector3<f32>,
    pub m_direction: Vector3<f32>,
}

impl Ray
{
    pub fn new(origin: Vector3<f32>, velocity: Vector3<f32>) -> Ray
    {
        Ray{ m_origin: origin, m_direction: velocity.normalize()}
    }
}