use crate::utils::color::Colorf;

pub mod imagewriter;

pub trait OutputManager
{
    fn writePixel(&self, x: i32, y: i32, colorf: Colorf);
}