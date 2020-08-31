use crate::geometry::{Geometry, GeomError};
use std::sync::Arc;
use cgmath::{Matrix3, SquareMatrix, Vector3, Matrix4};
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;
use std::fmt;

pub struct Instance
{
    m_geomptr: Arc<dyn Geometry>,
    m_inv_matrix: Matrix3<f32>,
    m_do_transform_texture: bool,
}

impl Instance
{
    pub fn new(geomptr: Arc<dyn Geometry>) -> Instance
    {
        Instance
        {
            m_geomptr: geomptr,
            m_inv_matrix: Matrix3::identity(),
            m_do_transform_texture: true,
        }
    }

    pub fn translate(&mut self, displace: Vector3<f32>)
    {

    }

    pub fn rotate(&mut self)
    {

    }
}

impl fmt::Debug for Instance
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Instance")
            .field("Inverse of Transformation Matrix", &self.m_inv_matrix)
            .finish()
    }
}

impl Geometry for Instance
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError> {
        unimplemented!()
    }
}