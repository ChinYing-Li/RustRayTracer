use cgmath::{Vector3, Zero, ElementWise};
use crate::geometry::{Geometry, Concrete, KEPSILON, GeomError};
use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::utils::shaderec::ShadeRec;
use crate::material::Material;
use crate::ray::Ray;
use std::fmt;
use crate::math::float_cmp;
use crate::math::float_cmp::{min, max};

#[derive(Clone)]
pub struct BBox
{
    pub m_vertex_0: Vector3<f32>,
    pub m_vertex_1: Vector3<f32>,
}

impl BBox
{
    pub fn new(vertex_0: Vector3<f32>, vertex_1: Vector3<f32>) -> BBox
    {
        BBox
        {
            m_vertex_0: vertex_0,
            m_vertex_1: vertex_1,
        }
    }

    pub fn union(&self, rhs: &BBox) -> BBox
    {
        let min_x = if self.m_vertex_0.x < rhs.m_vertex_0.x { self.m_vertex_0.x } else { self.m_vertex_1.x };
        let min_y = if self.m_vertex_0.y < rhs.m_vertex_0.y { self.m_vertex_0.y } else { self.m_vertex_1.y };
        let min_z = if self.m_vertex_0.z < rhs.m_vertex_0.x{ self.m_vertex_0.x } else { self.m_vertex_1.x};

        let max_x = if self.m_vertex_0.x > rhs.m_vertex_0.x{ self.m_vertex_0.x } else { self.m_vertex_1.x};
        let max_y = if self.m_vertex_0.y > rhs.m_vertex_0.y { self.m_vertex_0.y } else { self.m_vertex_1.y };
        let max_z = if self.m_vertex_0.z > rhs.m_vertex_0.x{ self.m_vertex_0.x } else { self.m_vertex_1.x};

        BBox::new(Vector3::new(min_x, min_y, min_z), Vector3::new(max_x, max_y, max_z))
    }

    /// Find the axis of which the bbox' dimension is largest.
    pub fn maximum_extent(&self) -> usize
    {
        let diag = self.get_diagonal();
        return if diag.x > diag.y && diag.x > diag.z { 0 } // axis x
                else if diag.y > diag.z { 1 } // axis y
                else { 2 } // axis z
    }

    pub fn get_diagonal(&self) -> Vector3<f32>
    {
        self.m_vertex_1 - self.m_vertex_0
    }

    pub fn get_surface_area(&self) -> f32
    {
        let diag = self.get_diagonal();
        2.0 * (diag.x * diag.y + diag.y * diag.z + diag.z * diag.x )
    }

    pub fn calculate_hit_time(&self, incomeray: &Ray, TMIN: &mut f32, TMAX: &mut f32) -> bool
    {
        let mut t_min = Vector3::zero();
        let mut t_max =  Vector3::zero();
        let INV_VEL = Vector3::new(1.0, 1.0, 1.0).div_element_wise(incomeray.m_direction);

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

        let mut t_min_max_component = max( t_min.x, max( t_min.y, t_min.z));
        let mut t_max_min_component = min(t_max.x, min( t_max.y, t_max.z ));

        if t_min_max_component < t_max_min_component && t_max_min_component > KEPSILON
        {
            *TMIN = t_min_max_component;
            *TMAX = t_max_min_component;
            return true;
        }
        false
    }
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
        let mut TMIN = 0.0_f32;
        let mut TMAX = 0.0_f32;
        let result = BBox::calculate_hit_time(self, incomeray, &mut TMIN, &mut TMAX);
        Ok(result)
    }
}

#[cfg(test)]
mod BBoxTest
{
    use cgmath::Vector3;
    use std::f32::INFINITY;
    use approx::{assert_relative_eq};

    use super::*;
    use crate::world::world::World;
    use crate::world::viewplane::ViewPlane;
    use crate::tracer::whitted::Whitted;
    use crate::sampler::mutijittered::MultiJittered;

    #[test]
    fn check_hit_small_x()
    {
        let v0 = Vector3::new(0.0, -5.0, 6.0);
        let v1 = Vector3::new(5.0, 0.0, 10.0);
        let bbox = BBox::new(v0, v1);

        let mut sampler = MultiJittered::new(256, 1);
        let vp = Box::new(ViewPlane::new(Arc::new(sampler)));
        let mut sr = ShadeRec::new(Arc::new(World::new(vp)));

        let ray = Ray::new(Vector3::new(-10.0, -10.0, 0.0),
                                Vector3::new(5.0, 3.5, 4.0));
        let mut t = INFINITY;
        let res = bbox.hit(&ray, &mut t,&mut sr);

        assert_eq!(res.unwrap(), true);
    }
}