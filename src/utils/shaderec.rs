use cgmath::prelude::*;
use cgmath::Vector3;
use std::{f32};
use std::option::Option;

use crate::ray::Ray;
use crate::world::world::World;
use std::sync::Arc;
use crate::utils::color::Colorf;
use std::f32::INFINITY;

#[derive(Clone, Copy, Debug)]
pub struct ShadeRec<'a>
{
    pub m_ishitting: bool,
    pub m_normal: Vector3<f32>,
    pub m_hitpoint: Vector3<f32>,
    pub m_ray: Ray,
    pub m_worldref: &'a World,
    pub m_color: Colorf,
    pub m_time: f32,
    pub m_depth: u16
}

impl<'a> ShadeRec<'a>
{
    pub fn new(worldref: &'a World) -> ShadeRec
    {
        ShadeRec{
            m_ishitting: false,
            m_normal: Vector3::new(0.0,0.0, 0.0),
            m_hitpoint: Vector3::new(0.0,0.0, 0.0),
            m_ray: Ray::new(Vector3::new(0.0, 0.0, 0.0),
                            Vector3::new(0.0, 0.0, 0.0)),
            m_worldref: worldref,
            m_color: Colorf::new(0.0, 0.0, 0.0),
            m_time: INFINITY,
            m_depth: 0,
        }
    }
}