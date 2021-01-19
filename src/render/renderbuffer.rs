use cgmath::{Vector3, InnerSpace, dot, ElementWise, Zero, Vector2};
use crate::utils::color::Colorf;

/// The buffer that stores our render result while we are rendering
pub struct RenderBuffer
{
    m_dim: Vector2<u32>,
    m_samples: Vec<Colorf>,
}

