use image;
use image::{Rgba, Pixel, RgbaImage, Rgb};
use crate::utils::color::{Colorf, Color8bit};
use crate::output::OutputManager;

pub struct ImageWriter<'a>
{
    m_imgdst: &'a str,
    m_imgbuffer: RgbaImage,
}

impl<'a> ImageWriter<'a>
{
    pub fn new(imgdst: &'a str, width: u32, height: u32) -> ImageWriter
    {
        ImageWriter{ m_imgdst: imgdst, m_imgbuffer: RgbaImage::new(width, height) }
    }
}

impl OutputManager for ImageWriter<'_>
{
    fn writePixel(&mut self, x: u32, y: u32, color: Colorf)
    {
        let convcolor = Color8bit::from(color);
        self.m_imgbuffer.put_pixel(x, y, Rgba([convcolor.m_r, convcolor.m_g, convcolor.m_b, 255]));
    }

    fn output(&self)
    {
        self.m_imgbuffer.save(self.m_imgdst).unwrap();
    }
}