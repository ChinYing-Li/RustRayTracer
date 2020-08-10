use crate::utils::color::Colorf;
use std::fmt;
use std::fmt::Formatter;

pub mod imagewriter;

pub trait OutputManager: fmt::Debug
{
    fn writePixel(&mut self, x: u32, y: u32, colorf: Colorf);
    fn output(&self);
}