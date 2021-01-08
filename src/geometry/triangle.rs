use cgmath::{Vector3, Zero, ElementWise, Matrix3, Transform, InnerSpace, SquareMatrix};
use crate::geometry::{Geometry, KEPSILON, Shadable, GeomError, Boundable};
use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::world::shaderec::ShadeRec;
use crate::material::Material;
use crate::ray::Ray;
use std::fmt;
use crate::utils::colorconstant::COLOR_BLACK;
use crate::geometry::bbox::BBox;
use crate::math::float_cmp::{max, min};

#[derive(Clone)]
pub struct Triangle
{
    pub m_vertex_0: Vector3<f32>,
    pub m_vertex_1: Vector3<f32>,
    pub m_vertex_2: Vector3<f32>,
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

    fn min_coordinate_on_axis(&self, axis: usize) -> f32
    {
        match axis
        {
            0 =>  min(min(self.m_vertex_0.x, self.m_vertex_1.x), self.m_vertex_2.x),
            1 => min(min(self.m_vertex_0.y, self.m_vertex_1.y), self.m_vertex_2.y),
            2 => min(min(self.m_vertex_0.z, self.m_vertex_1.z), self.m_vertex_2.z),
            _ => panic!("axis can only be of value 0, 1 or 2")
        }
    }

    fn max_coordinate_on_axis(&self, axis: usize) -> f32
    {
        match axis
        {
            0 =>  max(max(self.m_vertex_0.x, self.m_vertex_1.x), self.m_vertex_2.x),
            1 => max(max(self.m_vertex_0.y, self.m_vertex_1.y), self.m_vertex_2.y),
            2 => max(max(self.m_vertex_0.z, self.m_vertex_1.z), self.m_vertex_2.z),
            _ => panic!("axis can only be of value 0, 1 or 2")
        }
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
                                        incomeray.m_direction.x, incomeray.m_direction.y, incomeray.m_direction.z);
        let rhs = self.m_vertex_0 - incomeray.m_origin;
        let mut solution = Vector3::zero();

        match mat.invert()
        {
            Some(inverted_mat) => solution = inverted_mat * rhs,
            _ => return Err(GeomError::NoSolutionError)
        }
        //print!("{}, {}, {}", solution.x, solution.y, solution.z);;
        if solution.y < 0.0 || solution.x < 0.0 { return Ok(false) }
        if solution.x + solution.y > 1.0 { return Ok(false) }
        if solution.z < KEPSILON { return Ok(false) }

        match self.m_normals.len()
        {
            1 => shaderecord.m_normal = self.m_normals[0],
            3 => shaderecord.m_normal = self.interpolate_normal(solution.x, solution.y),
            _ => return Err(GeomError::WrongSizeError)
        }
        if solution.z < *time
        {
            *time = solution.z;
            shaderecord.m_hitpoint = incomeray.m_origin + solution.z * incomeray.m_direction;
            return Ok(true);
        }
        Ok(false)
    }
}

impl Boundable for Triangle
{
    fn get_bbox(&self) -> BBox
    {
        BBox::new(Vector3::new(self.min_coordinate_on_axis(0),
                                        self.min_coordinate_on_axis(1),
                                        self.min_coordinate_on_axis(2)),
            Vector3::new(self.max_coordinate_on_axis(0),
                         self.max_coordinate_on_axis(1),
                        self.max_coordinate_on_axis(2)))
    }
}

impl Shadable for Triangle
{
    fn get_material(&self) -> Arc<dyn Material> {
        if let Some(x) = self.m_material.clone() { x }
        else { panic!("The material for triangle is Not set") }
    }

    fn set_material(&mut self, material: Arc<dyn Material>)
    {
        self.m_material = Some(material);
    }

    fn shadow_hit(&self, shadow_ray: &Ray, tmin: &mut f32) -> bool
    {
        let v10 = self.m_vertex_0 - self.m_vertex_1;
        let v20 = self.m_vertex_0 - self.m_vertex_2;
        let mat = Matrix3::new(v10.x, v10.y, v10.z,
                               v20.x, v20.y, v20.z,
                               shadow_ray.m_direction.x, shadow_ray.m_direction.y, shadow_ray.m_direction.z);
        let rhs = self.m_vertex_0 - shadow_ray.m_origin;
        let mut solution = Vector3::zero();

        match mat.invert()
        {
            Some(inverted_mat) => solution = inverted_mat * rhs,
            _ => return false
        }
        //print!("{}, {}, {}", solution.x, solution.y, solution.z);;
        if solution.y < 0.0 || solution.x < 0.0 { return false }
        if solution.x + solution.y > 1.0 { return false }
        if solution.z < KEPSILON { return false }
        *tmin = solution.z;
        true
    }
}

#[cfg(test)]
mod TriangleTest
{
    use cgmath::Vector3;
    use std::f32::INFINITY;
    use approx::{assert_relative_eq};

    use super::*;
    use crate::world::viewplane::ViewPlane;
    use crate::tracer::whitted::Whitted;
    use crate::world::world::World;
    use crate::sampler::mutijittered::MultiJittered;

    #[test]
    fn check_hit()
    {
        let v0 = Vector3::new(0.0, 0.0, 0.0);
        let v1 = Vector3::new(0.0, 1.0, 0.0);
        let v2 = Vector3::new(0.5, 1.0, 1.0);
        let mut triangle = Triangle::new(v0, v1, v2);

        let mut sampler = MultiJittered::new(256, 1);
        let vp = Box::new(ViewPlane::new(Arc::new(sampler)));
        let mut sr = ShadeRec::new(Arc::new(World::new(vp, "whitted")));

        let ray = Ray::new(Vector3::new(0.3, 0.5, -1.0),
                                Vector3::new(0.01, 0.1, 1.2));
        let mut t = INFINITY;
        let res = triangle.hit(&ray, &mut t, &mut sr);

        assert_eq!(res.unwrap(), true);
        assert_relative_eq!(sr.m_normal, Vector3::new(0.8944271909999159, 0.0, -0.4472135954999579));
        assert_relative_eq!(t, 1.632815);
    }

    #[test]
    fn check_no_hit()
    {
        let v0 = Vector3::new(0.0, 0.0, 0.0);
        let v1 = Vector3::new(0.0, 1.0, 0.0);
        let v2 = Vector3::new(0.5, 1.0, 1.0);
        let triangle = Triangle::new(v0, v1, v2);
        let mut sampler = MultiJittered::new(256, 1);
        let vp = Box::new(ViewPlane::new(Arc::new(sampler)));
        let mut sr = ShadeRec::new(Arc::new(World::new(vp, "whitted")));

        let ray = Ray::new(Vector3::new(-1.0, 0.0, 0.0),
                           Vector3::new(-0.5, 1.0, 1.0));
        let mut t = 2.0;
        let res = triangle.hit(&ray, &mut t, &mut sr);

        assert_eq!(res.unwrap(), false);
    }

    #[test]
    fn testTriangleBBox()
    {
        let v0 = Vector3::new(1.0, 0.4, -1.0);
        let v1 = Vector3::new(2.0, 1.0, -10.0);
        let v2 = Vector3::new(0.5, 9.0, 1.0);
        let triangle = Triangle::new(v0, v1, v2);

        let bbox = triangle.get_bbox();
        assert_relative_eq!(bbox.m_vertex_0.x, 0.5);
        assert_relative_eq!(bbox.m_vertex_0.y, 0.4);
        assert_relative_eq!(bbox.m_vertex_1.z, 1.0);
    }
}
