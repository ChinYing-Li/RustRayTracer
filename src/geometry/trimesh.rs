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

/// The struct for storing all data in the .obj file. Should not be directly rendered
pub struct TriMesh
{
    m_vertex_position: Vec<Vector3<f32>>,
    m_mtl: Vec<obj::Mtl>, // Currently not supporting rendering materials defined in Mtl
    m_normals: Vec<Vector3<f32>>,
    m_texture: Vec<Vector2<f32>>,
    m_material: Option<Arc<dyn Material>>,
    pub m_bbox: BBox,
}

impl TriMesh
{
    pub fn new(objdata: &ObjData) -> TriMesh
    {
        let bbox_vert_0 = Vector3::new(objdata.position[0].iter()
                                           .fold(INFINITY, |max, &val| if val < max{ val } else{ max }),
                                       objdata.position[1].iter()
                                           .fold(INFINITY, |max, &val| if val < max{ val } else{ max }),
                                       objdata.position[2].iter()
                                           .fold(INFINITY, |max, &val| if val < max{ val } else{ max }));
        let bbox_vert_1 = Vector3::new(objdata.position[0].iter()
                                           .fold(-INFINITY, |max, &val| if val > max { val } else { max }),
                                       objdata.position[1].iter()
                                           .fold(-INFINITY, |max, &val| if val > max { val } else { max }),
                                       objdata.position[2].iter()
                                           .fold(-INFINITY, |max, &val| if val > max { val } else { max }));
        TriMesh
        {
            m_vertex_position: TriMesh::convert_to_vector3(&objdata.position),
            m_mtl: (*objdata).material_libs.clone(),
            m_normals: TriMesh::convert_to_vector3(&objdata.normal),
            m_texture: TriMesh::convert_to_vector2(&objdata.texture),
            m_material: None,
            m_bbox: BBox::new(bbox_vert_0, bbox_vert_1),
        }
    }

    pub fn set_material(&mut self, material_ptr: Arc<dyn Material>)
    {
        self.m_material = Some(material_ptr);
    }

    fn convert_to_vector3(v: &Vec<[f32; 3]>) -> Vec<Vector3<f32>>
    {
        v.iter().map(| position| Vector3::new(position[0], position[1], position[2])).collect()
    }

    fn convert_to_vector2(v: &Vec<[f32; 2]>) -> Vec<Vector2<f32>>
    {
        v.iter().map(| position| Vector2::new(position[0], position[1])).collect()
    }
}

pub fn create_meshtriangles(mesh_ptr: Arc<TriMesh>, objdata: &ObjData) -> Vec<MeshTriangle>
{
    let mut v = Vec::new();
    for object in objdata.objects.iter()
    {
        for group in object.groups.iter()
        {
            for poly in group.polys.iter()
            {
                v.push(MeshTriangle::new(
                    poly.0[0].0,
                    poly.0[1].0,
                    poly.0[2].0,
                    mesh_ptr.clone()));

                if poly.0.len() == 4
                {
                    v.push(MeshTriangle::new(poly.0[0].0,
                                             poly.0[2].0,
                                             poly.0[3].0,
                                             mesh_ptr.clone()));
                }
            }
        }
    }
    v
}

impl fmt::Debug for TriMesh
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("TriMesh")
            .finish()
    }
}