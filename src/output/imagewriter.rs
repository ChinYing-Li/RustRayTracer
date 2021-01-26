use image;
use image::{Rgba, Pixel, RgbaImage, Rgb};
use crate::utils::color::{Colorf, Color8bit};
use crate::output::OutputManager;
use std::fmt::Formatter;
use core::fmt;
use cgmath::Vector2;

pub struct ImageWriter<'a>
{
    pub m_imgpath: &'a str,
    m_imgresolution: (usize, usize),
    m_imgbuffer: RgbaImage,
}

impl ImageWriter<'_>
{
    pub fn new(imgpath: &str, width: usize, height: usize) -> ImageWriter
    {
        ImageWriter
        {
            m_imgpath: imgpath,
            m_imgresolution: (width, height),
            m_imgbuffer: RgbaImage::new(width as u32, height as u32)
        }
    }

    fn gamma_correction()
    {
        // TODO: make the gamma correction a function
    }
}

impl fmt::Debug for ImageWriter<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("ImageWriter")
            .field("image file destination", &self.m_imgpath)
            .field("image resolution", &self.m_imgresolution)
            .finish()
    }
}

impl OutputManager for ImageWriter<'_>
{
    fn get_img_dim(&self) -> (usize, usize)
    {
        self.m_imgresolution
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: Colorf, inv_gamma: f32)
    {
        let mut convcolor = Color8bit::from(color);
        convcolor.m_r = ((convcolor.m_r as f32) * inv_gamma.exp()) as u8;
        convcolor.m_g = ((convcolor.m_g as f32) * inv_gamma.exp()) as u8;
        convcolor.m_b = ((convcolor.m_b as f32) * inv_gamma.exp()) as u8;
        self.m_imgbuffer.put_pixel(x as u32, y as u32, Rgba([convcolor.m_r, convcolor.m_g, convcolor.m_b, 255]));
    }

    fn output(&mut self)
    {
        self.m_imgbuffer.save(self.m_imgpath).unwrap();
    }
}