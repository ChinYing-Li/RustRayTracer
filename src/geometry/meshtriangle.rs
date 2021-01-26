use cgmath::{Vector3, Zero, ElementWise, Matrix3, SquareMatrix, InnerSpace, Vector2};
use obj::{Obj, ObjData};
use std::cmp::{max, min};
use std::fmt;
use std::sync::Arc;

use crate::geometry::{Geometry, KEPSILON, Shadable, GeomError, Boundable};
use crate::utils::color::Colorf;
use crate::world::shaderec::ShadeRec;
use crate::math::float_cmp;
use crate::material::Material;
use crate::ray::Ray;
use crate::geometry::triangle::Triangle;
use crate::geometry::bbox::BBox;
use std::f32::INFINITY;
use crate::geometry::trimesh::TriMesh;


// Stores the indices of the face only
#[derive(Clone)]
pub struct MeshTriangle
{
    m_vertex_index: Vector3<usize>,
    m_mesh_ptr: Arc<TriMesh>,
}

impl MeshTriangle
{
    pub fn new(vertex0_index: usize, vertex1_index: usize, vertex2_index: usize, mesh_ptr: Arc<TriMesh>) -> MeshTriangle
    {
        MeshTriangle
        {
            m_vertex_index: Vector3::new(vertex0_index, vertex1_index, vertex2_index),
            m_mesh_ptr: mesh_ptr,
        }
    }

    fn interpolate_normal(&self, beta: f32, gamma: f32,
                          normal0: &Vector3<f32>, normal1: &Vector3<f32>, normal2: &Vector3<f32>) -> Vector3<f32>
    {
        ((1.0 - beta - gamma) * (*normal0)
            + beta * (*normal1)
            + gamma * (*normal2)).normalize()
    }

    fn min_coordinate_on_axis(&self, axis: u8) -> f32
    {
        let temp = float_cmp::min(self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[0]][axis as usize],
                                  self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[1]][axis as usize]);
        return float_cmp::min(temp, self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[2]][axis as usize]);
    }

    fn max_coordinate_on_axis(&self, axis: u8) -> f32
    {
        assert!(axis < 3);
        let temp = float_cmp::max(self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[0]][axis as usize],
                                  self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[1]][axis as usize]);
        return float_cmp::max(temp, self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[2]][axis as usize]);
    }
}

impl fmt::Debug for MeshTriangle
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Mesh triangle")
            .field("vertex 0 index", &self.m_vertex_index[0])
            .field("vertex 1 index", &self.m_vertex_index[1])
            .field("vertex 2 index", &self.m_vertex_index[2])
            .finish()
    }
}

impl Geometry for MeshTriangle
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        let vertex0 = &self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[0]];
        let vertex1 = &self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[1]];
        let vertex2 = &self.m_mesh_ptr.m_vertex_position[self.m_vertex_index[2]];

        let v10 = *vertex0 - *vertex1;
        let v20 = *vertex0 - *vertex2;
        let mat = Matrix3::new(v10.x, v10.y, v10.z,
                               v20.x, v20.y, v20.z,
                               incomeray.m_direction.x, incomeray.m_direction.y, incomeray.m_direction.z);
        let rhs = *vertex0 - incomeray.m_origin;
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

        let normal0 = &self.m_mesh_ptr.m_normals[self.m_vertex_index[0]];
        let normal1 = &self.m_mesh_ptr.m_normals[self.m_vertex_index[1]];
        let normal2 = &self.m_mesh_ptr.m_normals[self.m_vertex_index[2]];

        shaderecord.m_normal = self.interpolate_normal(solution.x, solution.y, normal0, normal1, normal2);
        *time = solution.z;
        shaderecord.m_hitpoint = incomeray.m_origin + solution.z * incomeray.m_direction;
        Ok(true)
    }
}

impl Shadable for MeshTriangle
{
    fn get_material(&self) -> Arc<dyn Material>
    {
        unimplemented!()
    }

    fn shadow_hit(&self, shadow_ray: &Ray, tmin: &mut f32) -> bool
    {
        let vertex0 = &self.m_mesh_ptr.m_vertex_position[self.m_vertex_index.x];
        let vertex1 = &self.m_mesh_ptr.m_vertex_position[self.m_vertex_index.y];
        let vertex2 = &self.m_mesh_ptr.m_vertex_position[self.m_vertex_index.z];

        let v10 = *vertex0 - *vertex1;
        let v20 = *vertex0 - *vertex2;

        let mat = Matrix3::new(v10.x, v10.y, v10.z,
                               v20.x, v20.y, v20.z,
                               shadow_ray.m_direction.x, shadow_ray.m_direction.y, shadow_ray.m_direction.z);
        let rhs = *(vertex0) - shadow_ray.m_origin;
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

impl Boundable for MeshTriangle
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