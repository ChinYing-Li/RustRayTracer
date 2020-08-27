use cgmath::{Vector3, Zero, ElementWise, Matrix3, Transform, InnerSpace, SquareMatrix};
use crate::geometry::{Geometry, KEPSILON, Shadable, GeomError};
use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::utils::shaderec::ShadeRec;
use crate::material::Material;
use crate::ray::Ray;
use std::fmt;
use std::cmp::{max, min};
use crate::utils::colorconstant::COLOR_BLACK;

pub struct Triangle
{
    pub m_vertex_0: Vector3<f32>,
    pub m_vertex_1: Vector3<f32>,
    pub m_vertex_2: Vector3<f32>,
    pub m_color: Colorf,
    m_normals: Vec<Vector3<f32>>, // the size of m_normals is either 1 or 3
}

impl Triangle
{
    pub fn new(vertex_0: Vector3<f32>, vertex_1: Vector3<f32>, vertex_2: Vector3<f32>) -> Triangle
    {
        Triangle
        {
            m_vertex_0: vertex_0,
            m_vertex_1: vertex_1,
            m_vertex_2: vertex_2,
            m_color: COLOR_BLACK,
            m_normals: Vec::with_capacity(3),
        }
    }

    fn interpolate_normal(&self, beta: f32, gamma: f32) -> Vector3<f32>
    {
        ((1.0 - beta - gamma) * self.m_vertex_0
            + beta * self.m_vertex_1
            + gamma * self.m_vertex_2).normalize()
    }
}

impl fmt::Debug for Triangle
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Triangle")
            .field("vertex 0", &self.m_vertex_0)
            .field("vertex 1", &self.m_vertex_1)
            .field("vertex 2", &self.m_vertex_2)
            .finish()
    }
}

impl Geometry for Triangle
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError> {
        let v10 = self.m_vertex_1 - self.m_vertex_0;
        let v20 = self.m_vertex_2 - self.m_vertex_0;
        let mat = Matrix3::new(v10.x, v20.x, incomeray.m_velocity.x,
                                        v10.y, v20.y, incomeray.m_velocity.y,
                                        v10.z, v20.z, incomeray.m_velocity.z);
        let rhs = self.m_vertex_0 - incomeray.m_origin;
        let solution = mat.invert().unwrap() * rhs;

        if solution.y < 0.0 { return Ok(false) }
        if solution.x + solution.y > 1.0 { return Ok(false) }
        if solution.z < KEPSILON { return Ok(false) }

        match self.m_normals.len()
        {
            1 => shaderecord.m_normal = self.m_normals[0],
            3 => shaderecord.m_normal = self.interpolate_normal(0.5, 0.5),
            _ => return Err(GeomError::WrongSizeError)
        }
        shaderecord.m_hitpoint = incomeray.m_origin + solution.z * incomeray.m_velocity;
        Ok(true)
    }
}

impl Shadable for Triangle
{
    fn get_color(&self) -> Colorf {
        self.m_color
    }

    fn set_color(&mut self, newcolor: Colorf) {
        unimplemented!()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        unimplemented!()
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        unimplemented!()
    }

    fn shadow_hit(&self, ray: &Ray, tmin: &mut f32) -> bool {
        unimplemented!()
    }
}