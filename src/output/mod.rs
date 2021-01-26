use crate::utils::color::Colorf;
use std::fmt;
use std::fmt::Formatter;

pub mod imagewriter;

pub trait OutputManager: fmt::Debug
{
    fn get_img_dim(&self) -> (usize, usize);
    fn write_pixel(&mut self, x: usize, y: usize, colorf: Colorf, inv_gamma: f32);
    fn output(&mut self);
}