use cgmath::{Vector3, Zero};
use std::{f32, fmt};
use std::option::Option;

use crate::ray::Ray;
use crate::world::world::World;
use std::sync::Arc;
use crate::utils::color::Colorf;
use std::f32::INFINITY;
use crate::geometry::Geometry;
use crate::light::Light;
use crate::material::Material;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct ShadeRec<'a>
{
    pub m_material: Option<&'a Material>,
    pub m_ishitting: bool,
    pub m_normal: Vector3<f32>,
    pub m_hitpoint: Vector3<f32>,
    pub m_local_hitpoint: Vector3<f32>, // For attaching texture
    pub m_ray: Ray, // For specular lights
    pub m_light_dir: Vector3<f32>, // For directional lights
    pub m_worldptr: Option<Rc<World<'a>>>,
    pub m_color: Colorf, // TODO: to be deprecated
    pub m_time: f32,
    pub m_depth: u16 // Recursion depth
}

impl<'a, 'b> ShadeRec<'a> where 'b: 'a
{
    pub fn new(worldptr: Rc<World<'b>>) -> ShadeRec<'a>
    {
        ShadeRec{
            m_material: None,
            m_ishitting: false,
            m_normal: Vector3::zero(),
            m_hitpoint: Vector3::zero(),
            m_local_hitpoint: Vector3::zero(),
            m_ray: Ray::new(Vector3::zero(), Vector3::zero()),
            m_light_dir: Vector3::zero(),
            m_worldptr: Some(worldptr),
            m_color: Colorf::new(0.0, 0.0, 0.0),
            m_time: INFINITY,
            m_depth: 0,
        }
    }
}

impl<'a> Debug for ShadeRec<'a>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShadeRec")
            .field("ishitting", &self.m_ishitting)
            .field("ray", &self.m_ray)
            .field("normal", &self.m_normal)
            .field("hitpoint", &self.m_hitpoint)
            .field("local hitpoint", &self.m_local_hitpoint)
            .finish()
    }
}