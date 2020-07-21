use cgmath::prelude::*;
use cgmath::Vector3;
use std::{f32};
use std::ops::{Add, Mul, Div, Sub, MulAssign, AddAssign, SubAssign};
use core::cmp::{max, min};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color
{
    m_r: f32,
    m_g: f32,
    m_b: f32,
}

impl Color
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

    pub fn new(r: f32, g: f32, b: f32) -> Color
    {
        Color{ m_r: r, m_g: g, m_b: b}
    }

    pub fn clamp(&self) -> Color
    {
        Color{ m_r: Color::clamp_elem(self.m_r), m_g: Color::clamp_elem(self.m_g), m_b: Color::clamp_elem(self.m_b)}
    }

}

impl Add for Color
{
    type Output = Color;
    /// Add two rgb color.
    fn add(self, rhs: Color) -> Color
    {
        let res = Color{ m_r: self.m_r + rhs.m_r, m_g: self.m_g + rhs.m_g, m_b: self.m_b + rhs.m_b};
        res.clamp()
    }
}

impl AddAssign for Color
{
    fn add_assign(&mut self, rhs: Color)
    {
        self.m_r += rhs.m_r;
        self.m_g += rhs.m_g;
        self.m_b += rhs.m_b;
        self.clamp();
    }
}

impl Mul for Color
{
    type Output = Color;
    /// Multiply two rgb color.
    fn mul(self, rhs: Color) -> Color
    {
        let res = Color{ m_r: self.m_r*rhs.m_r, m_g: self.m_g*rhs.m_g, m_b: self.m_b*rhs.m_b};
        res.clamp()
    }
}

impl MulAssign<f32> for Color
{
    fn mul_assign(&mut self, rhs: f32)
    {
        self.m_r *= rhs;
        self.m_g *= rhs;
        self.m_b *= rhs;
        self.clamp();
    }
}

impl Mul<f32> for Color
{
    type Output = Color;
    /// Multiply by a scalar
    fn mul(self, rhs: f32) -> Color
    {
        let res = Color{ m_r: self.m_r*rhs, m_g: self.m_g*rhs, m_b: self.m_b*rhs};
        res.clamp()
    }
}

impl Div<f32> for Color
{
    type Output = Color;
    /// Divide color by a scalar
    fn div(self, rhs: f32) -> Color
    {
        let res = Color{ m_r: self.m_r/rhs, m_g: self.m_g/rhs, m_b: self.m_b/rhs};
        res.clamp()
    }
}

impl Sub for Color
{
    type Output = Color;
    /// Subtract a rgb color from another rgn color.
    fn sub(self, rhs: Color) -> Color
    {
        let res = Color{ m_r: self.m_r-rhs.m_r, m_g: self.m_g-rhs.m_g, m_b: self.m_b-rhs.m_b};
        res.clamp()
    }
}

impl SubAssign for Color
{
    fn sub_assign(&mut self, rhs: Color)
    {
        self.m_r -= rhs.m_r;
        self.m_g -= rhs.m_g;
        self.m_b -= rhs.m_b;
        self.clamp();
    }
}

#[test]
fn Mulf32Test()
{
    let mut lhs = Color::new(0.5, 0.6, 0.7);
    lhs *= 0.2;
    let rhs = Color::new(0.1, 0.12, 0.14);
    assert_eq!(lhs, rhs);
}

#[test]
fn Mulf32ClampedTest()
{
    let mut lhs = Color::new(0.5, 0.6, 0.7);
    lhs *= 1.5;
    assert_eq!(lhs, Color::new(0.75, 0.9, 1.0));
}

#[test]
fn AddAssignTest()
{
    let mut lhs = Color::new(0.2, 0.3, 0.4);
    lhs += Color::new(0.2, 0.05, 0.03);
    assert_eq!(lhs, Color::new(0.4, 0.35, 0.43));
}