use crate::utils::color::Colorf;
use std::fmt;
use std::fmt::Formatter;

pub mod imagewriter;

pub trait OutputManager: fmt::Debug
{
    fn writePixel(&mut self, x: u16, y: u16, colorf: Colorf);
    fn output(&mut self);
}