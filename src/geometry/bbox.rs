use cgmath::{Vector3, Zero, ElementWise};
use crate::geometry::{Geometry, Concrete, KEPSILON, GeomError};
use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::utils::shaderec::ShadeRec;
use crate::material::Material;
use crate::ray::Ray;
use std::fmt;
use std::cmp::{max, min};

pub struct BBox
{
    pub m_vertex_0: Vector3<f32>,
    pub m_vertex_1: Vector3<f32>,
}

impl fmt::Debug for BBox
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("BBox")
            .field("Vector 0", &self.m_vertex_0)
            .field("Vector 1", &self.m_vertex_1)
            .finish()
    }
}

impl Geometry for BBox
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        let mut t_min = Vector3::zero();
        let mut t_max =  Vector3::zero();
        let INV_VEL = Vector3::new(1.0, 1.0, 1.0).div_element_wise(incomeray.m_velocity);

        if INV_VEL.x >= 0.0
        {
            t_min.x = (self.m_vertex_0.x - incomeray.m_origin.x) * INV_VEL.x;
            t_max.x = (self.m_vertex_1.x - incomeray.m_origin.x) * INV_VEL.x;
        }
        else
        {
            t_min.x = (self.m_vertex_1.x - incomeray.m_origin.x) * INV_VEL.x;
            t_max.x = (self.m_vertex_0.x - incomeray.m_origin.x) * INV_VEL.x;
        }

        if INV_VEL.y >= 0.0
        {
            t_min.y = (self.m_vertex_0.y - incomeray.m_origin.y) * INV_VEL.y;
            t_max.y = (self.m_vertex_1.y - incomeray.m_origin.y) * INV_VEL.y;
        }
        else
        {
            t_min.y = (self.m_vertex_1.y - incomeray.m_origin.y) * INV_VEL.y;
            t_max.y = (self.m_vertex_0.y - incomeray.m_origin.y) * INV_VEL.y;
        }

        if INV_VEL.z >= 0.0
        {
            t_min.z = (self.m_vertex_0.z - incomeray.m_origin.z) * INV_VEL.z;
            t_max.z = (self.m_vertex_1.z - incomeray.m_origin.z) * INV_VEL.z;
        }
        else
        {
            t_min.z = (self.m_vertex_1.z - incomeray.m_origin.z) * INV_VEL.z;
            t_max.z = (self.m_vertex_0.z - incomeray.m_origin.z) * INV_VEL.z;
        }

        let mut t_min_max_component = if t_min.x >= t_min.y { t_min.x } else { t_min.y };
        t_min_max_component = if t_min.z >= t_min_max_component { t_min.z } else { t_min_max_component };
        let mut t_max_min_component = if t_max.x <= t_max.y { t_max.x } else { t_min.y };
        t_min_max_component = if t_min.z <= t_min_max_component { t_min.z } else { t_min_max_component };

        Ok(t_min_max_component < t_max_min_component && t_max_min_component > KEPSILON)
    }
}