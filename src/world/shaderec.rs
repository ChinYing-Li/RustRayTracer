use cgmath::{Vector3, Zero};
use std::{f32, fmt};
use std::option::Option;
use std::sync::Arc;

use crate::ray::Ray;
use crate::world::world::World;
use crate::material::Material;


#[derive(Clone)]
pub struct ShadeRec
{
    pub m_material: Option<Arc<dyn Material>>,
    pub m_hit: bool,
    pub m_normal: Vector3<f32>,
    pub m_hitpoint: Vector3<f32>,
    pub m_local_hitpoint: Vector3<f32>, // For attaching texture
    pub m_ray: Ray, // For specular lights
    pub m_light_dir: Vector3<f32>, // For directional lights
    pub m_worldptr: Arc<World>,
    pub m_time: f32,
    pub m_depth: u16 // Recursion depth
}

impl ShadeRec
{
    pub fn new(world: Arc<World>) -> ShadeRec
    {
        ShadeRec{
            m_material: None,
            m_hit: false,
            m_normal: Vector3::zero(),
            m_hitpoint: Vector3::zero(),
            m_local_hitpoint: Vector3::zero(),
            m_ray: Ray::new(Vector3::zero(), Vector3::zero()),
            m_light_dir: Vector3::zero(),
            m_worldptr: world.clone(),
            m_time: f32::INFINITY,
            m_depth: 0,
        }
    }

    pub fn set_world(&mut self, worldptr: Arc<World>)
    {
        self.m_worldptr = worldptr.clone();
    }

    pub fn get_dummy() -> ShadeRec
    {
        ShadeRec::new(Arc::new(World::get_dummy()))
    }
}

impl fmt::Debug for ShadeRec
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShadeRec")
            .field("hit", &self.m_hit)
            .field("ray", &self.m_ray)
            .field("normal", &self.m_normal)
            .field("hitpoint", &self.m_hitpoint)
            .field("local hitpoint", &self.m_local_hitpoint)
            .finish()
    }
}