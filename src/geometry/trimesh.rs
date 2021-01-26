use cgmath::{Vector3, Vector2};
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
use crate::geometry::meshtriangle::MeshTriangle;

/// The struct for storing all data in the .obj file. Should not be directly rendered
pub struct TriMesh
{
    pub m_vertex_position: Vec<Vector3<f32>>,
    m_mtl: Vec<obj::Mtl>, // Currently not supporting rendering materials defined in Mtl
    pub m_normals: Vec<Vector3<f32>>,
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