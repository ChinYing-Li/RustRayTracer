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
    pub triangles: Vec<MeshTriangle>,
}

pub struct MeshTriangle
{
    
}