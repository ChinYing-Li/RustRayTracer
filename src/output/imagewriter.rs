use image;
use image::{Rgba, Pixel, RgbaImage, Rgb};
use crate::utils::color::{Colorf, Color8bit};

pub fn writePixel( img: &mut RgbaImage, x: u32, y: u32, color: Colorf
)
{
    let convcolor = Color8bit::from(color);
    img.put_pixel(x, y, Rgba([convcolor.m_r, convcolor.m_g, convcolor.m_b, 255]));
}