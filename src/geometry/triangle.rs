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
    pub m_material: Option<Arc<dyn Material>>,
    m_normals: Vec<Vector3<f32>>, // the size of m_normals is either 1 or 3
}

impl Triangle
{
    // vertex listed in clockwise fashion
    pub fn new(vertex_0: Vector3<f32>, vertex_1: Vector3<f32>, vertex_2: Vector3<f32>) -> Triangle
    {
        // create one normal by default
        let default_normal = (vertex_1 - vertex_0).cross(vertex_2 - vertex_0).normalize();
        let mut normal_vec = Vec::with_capacity(3);
        normal_vec.push(default_normal);

        Triangle
        {
            m_vertex_0: vertex_0,
            m_vertex_1: vertex_1,
            m_vertex_2: vertex_2,
            m_color: COLOR_BLACK,
            m_material: None,
            m_normals: normal_vec,
        }
    }

    fn interpolate_normal(&self, beta: f32, gamma: f32) -> Vector3<f32>
    {
        ((1.0 - beta - gamma) * self.m_vertex_0
            + beta * self.m_vertex_1
            + gamma * self.m_vertex_2).normalize()
    }

    fn set_normal(&mut self, normal_index: usize, new_normal: Vector3<f32>)
    {
        if normal_index > 2
        {
            panic!("A trianlge can't have more then 3 vertices, but the index is\
            none of 0, 1 or 2");
        }
        self.m_normals[normal_index] = new_normal;
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
        let v10 = self.m_vertex_0 - self.m_vertex_1;
        let v20 = self.m_vertex_0 - self.m_vertex_2;
        let mat = Matrix3::new(v10.x, v10.y, v10.z,
                                        v20.x, v20.y, v20.z,
                                        incomeray.m_velocity.x, incomeray.m_velocity.y, incomeray.m_velocity.z);
        let rhs = self.m_vertex_0 - incomeray.m_origin;
        let mut solution = Vector3::zero();

        match mat.invert()
        {
            Some(inverted_mat) => solution = inverted_mat * rhs,
            _ => return Err(GeomError::NoSolutionError)
        }
        print!("{}, {}, {}", solution.x, solution.y, solution.z);;
        if solution.y < 0.0 || solution.x < 0.0 { return Ok(false) }
        if solution.x + solution.y > 1.0 { return Ok(false) }
        if solution.z < KEPSILON { return Ok(false) }

        match self.m_normals.len()
        {
            1 => shaderecord.m_normal = self.m_normals[0],
            3 => shaderecord.m_normal = self.interpolate_normal(0.5, 0.5),
            _ => return Err(GeomError::WrongSizeError)
        }
        *time = solution.z;
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
        self.m_color = newcolor;
    }

    fn get_material(&self) -> Arc<dyn Material> {
        if let Some(x) = self.m_material.clone() { x }
        else { panic!("The material for sphere is Not set") }
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.m_material = Some(material.clone());
    }

    fn shadow_hit(&self, ray: &Ray, tmin: &mut f32) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod TriangleTest
{
    use cgmath::Vector3;
    use std::f32::INFINITY;
    use approx::{assert_relative_eq};

    use super::*;

    #[test]
    fn check_hit()
    {
        let v0 = Vector3::new(0.0, 0.0, 0.0);
        let v1 = Vector3::new(0.0, 1.0, 0.0);
        let v2 = Vector3::new(0.5, 1.0, 1.0);
        let mut triangle = Triangle::new(v0, v1, v2);

        let mut sr = ShadeRec::new();
        let ray = Ray::new(Vector3::new(0.3, 0.5, -1.0),
                                Vector3::new(0.01, 0.1, 1.2));
        let mut t = INFINITY;
        let res = triangle.hit(&ray, &mut t, &mut sr);

        assert_eq!(res.unwrap(), true);
        assert_relative_eq!(sr.m_normal, Vector3::new(0.8944271909999159, 0.0, -0.4472135954999579));
        assert_relative_eq!(t, 1.3559322033898307);
    }

    #[test]
    fn check_no_hit()
    {
        let v0 = Vector3::new(0.0, 0.0, 0.0);
        let v1 = Vector3::new(0.0, 1.0, 0.0);
        let v2 = Vector3::new(0.5, 1.0, 1.0);
        let triangle = Triangle::new(v0, v1, v2);
        let mut sr = ShadeRec::new();
        let ray = Ray::new(Vector3::new(-1.0, 0.0, 0.0),
                           Vector3::new(-0.5, 1.0, 1.0));
        let mut t = 2.0;
        let res = triangle.hit(&ray, &mut t, &mut sr);

        assert_eq!(res.unwrap(), false);
    }
}
