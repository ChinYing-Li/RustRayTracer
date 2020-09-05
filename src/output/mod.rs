use crate::utils::color::Colorf;
use std::fmt;
use std::fmt::Formatter;

pub mod imagewriter;

pub trait OutputManager: fmt::Debug
{
    fn write_pixel(&mut self, x: u16, y: u16, colorf: Colorf, inv_gamma: f32);
    fn output(&mut self);
}