use std::{f32, convert::{From, Into}};
use std::ops::{Add, Mul, Div, Sub, MulAssign, AddAssign, SubAssign, DivAssign};

use crate::utils::colorconstant::e;

const INV_255 : f32 = 1.0 / 255.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colorf
{
    pub m_r: f32,
    pub m_g: f32,
    pub m_b: f32,
}

// Colorf values are float numbers within the range [0.0, 1.0]
impl Colorf
{
    fn clamp_elem(val: f32) -> f32
    {
        match Some(val)
        {
            Some(val) if Some(val) < Some(0.0) => 0.0,
            Some(val) if Some(val) > Some(1.0) => 1.0,
            _ => val,
        }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Colorf
    {
        Colorf{ m_r: r, m_g: g, m_b: b}
    }

    pub fn clamp(&self) -> Colorf
    {
        Colorf{ m_r: Colorf::clamp_elem(self.m_r), m_g: Colorf::clamp_elem(self.m_g), m_b: Colorf::clamp_elem(self.m_b)}
    }

    pub fn color_filter_exponential(&self, t: f32) -> Colorf
    {
        Colorf::new(e.powf(-self.m_r * t), e.powf(-self.m_g * t), e.powf(-self.m_b * t))
    }
}

impl Add for Colorf
{
    type Output = Colorf;
    /// Add two rgb color.
    fn add(self, rhs: Colorf) -> Colorf
    {
        Colorf{ m_r: self.m_r + rhs.m_r, m_g: self.m_g + rhs.m_g, m_b: self.m_b + rhs.m_b}
    }
}

impl AddAssign for Colorf
{
    fn add_assign(&mut self, rhs: Colorf)
    {
        self.m_r += rhs.m_r;
        self.m_g += rhs.m_g;
        self.m_b += rhs.m_b;
    }
}

impl Mul for Colorf
{
    type Output = Colorf;
    /// Multiply two rgb color.
    fn mul(self, rhs: Self) -> Colorf
    {
        Colorf{ m_r: self.m_r * rhs.m_r, m_g: self.m_g * rhs.m_g, m_b: self.m_b * rhs.m_b}
    }
}

impl MulAssign for Colorf
{
    fn mul_assign(&mut self, rhs: Self) {
        self.m_r *= rhs.m_r;
        self.m_g *= rhs.m_g;
        self.m_b *= rhs.m_b;
    }
}

impl MulAssign<f32> for Colorf
{
    fn mul_assign(&mut self, rhs: f32)
    {
        self.m_r *= rhs;
        self.m_g *= rhs;
        self.m_b *= rhs;
    }
}

impl Mul<f32> for Colorf
{
    type Output = Colorf;
    /// Multiply by a scalar
    fn mul(self, rhs: f32) -> Colorf
    {
        Colorf{ m_r: self.m_r * rhs, m_g: self.m_g * rhs, m_b: self.m_b * rhs}
    }
}

impl Div<f32> for Colorf
{
    type Output = Colorf;
    /// Divide color by a scalar
    fn div(self, rhs: f32) -> Colorf
    {
        let rhs_inv = 1.0f32 / rhs;
        Colorf{ m_r: self.m_r * rhs_inv, m_g: self.m_g * rhs_inv, m_b: self.m_b * rhs_inv}
    }
}

impl DivAssign<f32> for Colorf
{
    /// Divide and assign color by a scalar
    fn div_assign(&mut self, rhs: f32) {
        self.m_r *= rhs;
        self.m_g *= rhs;
        self.m_b *= rhs;
    }
}

impl Sub for Colorf
{
    type Output = Colorf;
    /// Subtract a rgb color from another rgn color.
    fn sub(self, rhs: Colorf) -> Colorf
    {
        Colorf{ m_r: self.m_r-rhs.m_r, m_g: self.m_g-rhs.m_g, m_b: self.m_b-rhs.m_b}
    }
}

impl SubAssign for Colorf
{
    fn sub_assign(&mut self, rhs: Colorf)
    {
        self.m_r -= rhs.m_r;
        self.m_g -= rhs.m_g;
        self.m_b -= rhs.m_b;
    }
}

impl From<Color8bit> for Colorf
{
    fn from(src: Color8bit) -> Colorf
    {
        Colorf::new(src.m_r as f32 * INV_255, src.m_g as f32 * INV_255, src.m_b as f32 * INV_255).clamp()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color8bit
{
    pub m_r: u8,
    pub m_g: u8,
    pub m_b: u8,
}

impl Color8bit
{
    pub fn new(r: u8, g: u8, b: u8) -> Color8bit
    {
        Color8bit{ m_r: r, m_g: g, m_b: b}
    }

    pub fn clamp(&mut self)
    {
        // TODO: Clamp method for Color8bit
    }
}

impl From<Colorf> for Color8bit
{
    fn from(src: Colorf) -> Color8bit
    {
        Color8bit::new((src.m_r * 255.0).round() as u8,
                       (src.m_g * 255.0).round() as u8,
                       (src.m_b * 255.0).round() as u8)
    }
}

#[cfg(test)]
mod ColorTest
{
    use super::*;
    use approx::{assert_relative_eq};

    /*
    fn Mulf32Test()
    {
        let mut lhs = Colorf::new(0.5, 0.6, 0.7);
        lhs *= 0.2;
        let rhs = Colorf::new(0.1, 0.12, 0.14);
        assert_relative_eq!(lhs, rhs);
    }

    #[test]
    fn Mulf32ClampedTest()
    {
        let mut lhs = Colorf::new(0.5, 0.6, 0.7);
        lhs *= 1.5;
        assert_relative_eq!(lhs, Colorf::new(0.75, 0.9, 1.0));
    }
    */

    #[test]
    fn add_assign_test()
    {
        let mut lhs = Colorf::new(0.2, 0.3, 0.4);
        lhs += Colorf::new(0.2, 0.05, 0.03);
        assert_relative_eq!(lhs.m_r, 0.4, epsilon=f32::EPSILON);
    }

    #[test]
    fn colorf2color8bit()
    {
        let src = Colorf::new(0.45, 0.57, 0.89);
        let dst = Color8bit::from(src);
        assert_eq!(dst, Color8bit::new(115, 145, 227));
    }

    #[test]
    fn Color8bit2Colorf()
    {
        let src = Color8bit::new(10, 160, 244);
        let dst = Colorf::from(src);
        assert_relative_eq!(dst.m_r, 0.0392156862745098, epsilon=f32::EPSILON);
    }
}
