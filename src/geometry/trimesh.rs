use cgmath::{Vector3, Zero, ElementWise};
use crate::geometry::{Geometry, KEPSILON, Shadable, GeomError};
use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::utils::shaderec::ShadeRec;
use crate::material::Material;
use crate::ray::Ray;
use std::fmt;
use std::cmp::{max, min};
use crate::geometry::triangle::Triangle;

pub struct TriMesh
{
}

impl TriMesh
{
    pub fn new() -> TriMesh
    {
        TriMesh
        {

        }
    }
}

impl fmt::Debug for TriMesh
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("TriMesh")
            .finish()
        // .field("number of vertices", &self.m_num_vertices)
        //.field("number of triangles", &self.m_num_triangles)
    }
}

impl Geometry for TriMesh
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError> {
        unimplemented!()
    }
}