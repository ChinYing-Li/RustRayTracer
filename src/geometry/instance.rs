use crate::geometry::{Geometry, GeomError, Boundable, BoundedConcrete};
use std::sync::Arc;
use cgmath::{Matrix3, SquareMatrix, Vector3, Matrix4, InnerSpace, ElementWise};
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;
use std::fmt;
use std::ptr::null;
use crate::material::Material;
use crate::geometry::bbox::BBox;
use std::f32::INFINITY;

#[derive(Clone)]
pub struct Instance
{
    m_geomptr: Arc<dyn BoundedConcrete>,
    m_material_ptr: Arc<dyn Material>,
    m_inv_matrix: Matrix3<f32>,
    m_forward_matrix: Matrix3<f32>,
    m_do_transform_texture: bool,
    m_bbox: BBox,
}

impl Instance
{
    pub fn new(geomptr: Arc<dyn BoundedConcrete>) -> Instance
    {
        Instance
        {
            m_geomptr: geomptr.clone(),
            m_material_ptr: geomptr.get_material(),
            m_inv_matrix: Matrix3::identity(),
            m_forward_matrix: Matrix3::identity(),
            m_do_transform_texture: true,
            m_bbox: geomptr.as_ref().get_bbox(),
        }
    }

    pub fn set_geom(&mut self, new_geom_ptr: Arc<dyn BoundedConcrete>, use_material: bool)
    {
        self.m_geomptr = new_geom_ptr.clone();
        if use_material
        {
            self.m_material_ptr = self.m_geomptr.get_material();
        }
        self.compute_bbox();
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
        let mut inverted_ray = Ray::new(self.m_inv_matrix * incomeray.m_origin, self.m_inv_matrix * incomeray.m_direction);

        if self.m_geomptr.hit(&inverted_ray, time, shaderecord)
            .unwrap_or(false)
        {
            shaderecord.m_normal = self.m_inv_matrix * shaderecord.m_normal;
            shaderecord.m_normal.normalize();

            if !self.m_do_transform_texture
            {
                shaderecord.m_hitpoint = incomeray.m_origin + incomeray.m_direction.mul_element_wise(*time);
            }
            return Ok(true);
        }
        Ok(false)
    }
}

impl Boundable for Instance
{
    fn compute_bbox(&mut self)
    {
        self.m_bbox = self.m_geomptr.get_bbox();
        let trans_vertices: [Vector3<f32>; 8] = [self.m_forward_matrix * self.m_bbox.m_vertex_0,
            self.m_forward_matrix * Vector3::new(self.m_bbox.m_vertex_1.x,
                                                 self.m_bbox.m_vertex_0.y,
                                                 self.m_bbox.m_vertex_0.z),
            self.m_forward_matrix * Vector3::new(self.m_bbox.m_vertex_0.x,
                                                 self.m_bbox.m_vertex_1.y,
                                                 self.m_bbox.m_vertex_0.z),
            self.m_forward_matrix * Vector3::new(self.m_bbox.m_vertex_0.x,
                                                 self.m_bbox.m_vertex_0.y,
                                                 self.m_bbox.m_vertex_1.z),
            self.m_forward_matrix * Vector3::new(self.m_bbox.m_vertex_1.x,
                                                 self.m_bbox.m_vertex_1.y,
                                                 self.m_bbox.m_vertex_0.z),
            self.m_forward_matrix * Vector3::new(self.m_bbox.m_vertex_1.x,
                                                 self.m_bbox.m_vertex_0.y,
                                                 self.m_bbox.m_vertex_1.z),
            self.m_forward_matrix * Vector3::new(self.m_bbox.m_vertex_0.x,
                                                 self.m_bbox.m_vertex_1.y,
                                                 self.m_bbox.m_vertex_1.z),
            self.m_forward_matrix * self.m_bbox.m_vertex_1];

        let mut min_x = INFINITY;
        let mut min_y = INFINITY;
        let mut min_z = INFINITY;

        let mut max_x = -INFINITY;
        let mut max_y = -INFINITY;
        let mut max_z = -INFINITY;

        for v in trans_vertices.iter()
        {
            if v.x < min_x { min_x = v.x } else if v.x > max_x { max_x = v.x };
            if v.y < min_y { min_y = v.y } else if v.y > max_y { max_y = v.y };
            if v.z < min_z { min_z = v.z } else if v.z > max_z { max_z = v.z };
        }

        self.m_bbox = BBox::new(Vector3::new(min_x, min_y, min_z), Vector3::new(max_x, max_y, max_z))
    }

    fn get_bbox(&self) -> BBox
    {
        self.m_bbox.clone()
    }
}