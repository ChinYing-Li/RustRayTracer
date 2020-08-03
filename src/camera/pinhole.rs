use crate::camera::{CamStruct, Camera};
use crate::world::world::World;
use crate::utils::color::Colorf;

pub struct Pinhole
{
    pub core: CamStruct,
}

impl Camera for Pinhole
{
    fn renderScene(&mut self, worldref: &World)
    {
        let clr = Colorf::new(0.0, 0.0, 0.0);

    }
}