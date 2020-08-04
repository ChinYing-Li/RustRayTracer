use crate::utils::color::Colorf;

pub mod imagewriter;

pub trait OutputManager
{
    fn writePixel(&mut self, x: u32, y: u32, colorf: Colorf);
    fn output(&self);
}