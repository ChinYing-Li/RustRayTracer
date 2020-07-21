use cgmath::prelude::*;
use cgmath::Vector3;
use std::{f32};
use std::option::Option;

use crate::ray::Ray;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadeRec
{
    pub m_ishitting: bool,
    pub m_normal: Vector3<f32>,
    pub m_hitpoint: Vector3<f32>,

}

impl ShadeRec
{
    pub fn new() -> ShadeRec
    {
        ShadeRec{
            m_ishitting: false,
            m_normal: Vector3::new(0.0,0.0, 0.0),
            m_hitpoint: Vector3::new(0.0,0.0, 0.0),
        }
    }


}