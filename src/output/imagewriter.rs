use image;
use image::{Rgba, Pixel, RgbaImage, Rgb};
use crate::utils::color::{Colorf, Color8bit};
use crate::output::OutputManager;
use std::fmt::Formatter;
use core::fmt;
use cgmath::Vector2;

pub struct ImageWriter<'a>
{
    m_imgdst: &'a str,
    m_imgresolution: Vector2<u16>,
    m_imgbuffer: RgbaImage,
}

impl ImageWriter<'_>
{
    pub fn new<'a>(imgdst: &'a str, width: u16, height: u16) -> ImageWriter
    {
        ImageWriter{ m_imgdst: imgdst,
                    m_imgresolution: Vector2::new(width, height),
                    m_imgbuffer: RgbaImage::new(width.into(), height.into()) }
    }
}

impl fmt::Debug for ImageWriter<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("ImageWriter")
            .field("image file destination", &self.m_imgdst)
            .field("image resolution", &self.m_imgresolution)
            .finish()
    }
}

impl OutputManager for ImageWriter<'_>
{
    fn writePixel(&mut self, x: u16, y: u16, color: Colorf)
    {
        let convcolor = Color8bit::from(color);
        self.m_imgbuffer.put_pixel(x.into(), y.into(), Rgba([convcolor.m_r, convcolor.m_g, convcolor.m_b, 255]));
    }

    fn output(&mut self)
    {
        self.m_imgbuffer.save(self.m_imgdst).unwrap();
    }
}