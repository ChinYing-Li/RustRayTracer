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
    pub fn new(imgdst: &str, width: u16, height: u16) -> ImageWriter
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
    fn write_pixel(&mut self, x: u16, y: u16, color: Colorf, inv_gamma: f32)
    {
        let mut convcolor = Color8bit::from(color);
        convcolor.m_r = ((convcolor.m_r as f32) * inv_gamma.exp()) as u8;
        convcolor.m_g = ((convcolor.m_g as f32) * inv_gamma.exp()) as u8;
        convcolor.m_b = ((convcolor.m_b as f32) * inv_gamma.exp()) as u8;
        self.m_imgbuffer.put_pixel(x.into(), y.into(), Rgba([convcolor.m_r, convcolor.m_g, convcolor.m_b, 255]));
    }

    fn output(&mut self)
    {
        self.m_imgbuffer.save(self.m_imgdst).unwrap();
    }
}