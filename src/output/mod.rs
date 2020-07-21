use crate::utils::color::Color;

pub mod imagewriter;

pub trait OutputManager
{
    fn writePixel(&self, x: i32, y: i32, color: Color);
}