use cgmath::{Vector3, Zero, ElementWise, Matrix3, SquareMatrix, InnerSpace};
use crate::geometry::{Geometry, KEPSILON, Shadable, GeomError};
use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::utils::shaderec::ShadeRec;
use crate::material::Material;
use crate::ray::Ray;
use std::fmt;
use std::cmp::{max, min};
use crate::geometry::triangle::Triangle;
use obj::{Obj, ObjData};

pub struct TriMesh
{
    m_path_to_obj: String,
    m_objects: Vec<obj::Object>,
    m_vertex_position: Vec<[f32; 3]>,
    m_material: Vec<obj::Mtl>,
    m_normals: Vec<[f32; 3]>,
    m_texture: Vec<[f32; 2]>,
    //m_triangles: Option<Vec<Triangles>>,
}

impl TriMesh
{
    pub fn new(path_to_obj: String, path_to_mtl: String) -> TriMesh
    {
        let temp_objdata = Obj::load(path_to_obj)
            .unwrap_or(panic!("The path is not valid; can't load .obj file")).data;
        TriMesh
        {
            m_path_to_obj: path_to_obj.clone(),
            m_objects: temp_objdata.objects,
            m_vertex_position: temp_objdata.position,
            m_material: temp_objdata.material_libs,
            m_normals: temp_objdata.normal,
            m_texture: temp_objdata.texture,
        }
    }

    fn triangle_hit(&self, vertex0: &Vector3<f32>, vertex1: &Vector3<f32>, vertex2: &Vector3<f32>,
                            normal0: &Vector3<f32>, normal1: &Vector3<f32>, normal2: &Vector3<f32>,
                            incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        let v10 = *vertex0 - *vertex1;
        let v20 = *vertex0 - *vertex2;
        let mat = Matrix3::new(v10.x, v10.y, v10.z,
                               v20.x, v20.y, v20.z,
                               incomeray.m_direction.x, incomeray.m_direction.y, incomeray.m_direction.z);
        let rhs = vertex0 - incomeray.m_origin;
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

        shaderecord.m_normal = self.interpolate_normal(solution.x, solution.y, normal0, normal1, normal2);
        *time = solution.z;
        shaderecord.m_hitpoint = incomeray.m_origin + solution.z * incomeray.m_direction;
        Ok(true)
    }

    fn interpolate_normal(&self, beta: f32, gamma: f32,
                          normal0: &Vector3<f32>, normal1: &Vector3<f32>, normal2: &Vector3<f32>) -> Vector3<f32>
    {
        ((1.0 - beta - gamma) * (*normal0)
            + beta * (*normal1)
            + gamma * (*normal2)).normalize()
    }
}

/// Stores the indices of the face only
struct MeshTriangle
{
    m_vertex0_index: usize,
    m_vertex1_index: usize,
    m_vertex2_index: usize,
}

impl fmt::Debug for TriMesh
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("TriMesh")
            .finish()
    }
}

impl Geometry for TriMesh
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        unimplemented!()
    }
}

/*
 for polys in self.m_objects.iter().map(|obj| obj.groups.iter())
            .map(|g| g.polys)
        {
            for poly in polys
            {
                self.triangle_hit();
            }
        }
*/

impl Shadable for TriMesh
{
    fn get_material(&self) -> Arc<dyn Material>
    {
        unimplemented!()
    }

    fn shadow_hit(&self, ray: &Ray, tmin: &mut f32) -> bool {
        unimplemented!()
    }
}