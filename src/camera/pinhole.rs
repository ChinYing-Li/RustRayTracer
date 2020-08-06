use crate::camera::{CamStruct, Camera};
use crate::world::world::World;
use crate::utils::color::Colorf;
use cgmath::Vector3;

pub struct Pinhole
{
    pub m_core: CamStruct,
}

impl Pinhole
{
    pub fn new(eye: Vector3<f32>, lookat: Vector3<f32>, up: Vector3<f32>) -> Pinhole
    {
        let mut core = CamStruct::new(eye, lookat, up);
        core.ComputeUVW();
        Pinhole{ m_core: core}
    }
}

impl Camera for Pinhole
{
    fn renderScene(&mut self, worldref: &World)
    {
        let clr = Colorf::new(0.0, 0.0, 0.0);
    }
}