use cgmath::{Vector3, Zero, ElementWise};
use crate::geometry::{Geometry, KEPSILON, Shadable, GeomError, Boundable};
use crate::utils::color::Colorf;
use std::sync::Arc;
use crate::utils::shaderec::ShadeRec;
use crate::material::Material;
use crate::ray::Ray;
use std::fmt;
use crate::math::float_cmp;
use crate::geometry::bbox::BBox;
use core::num::FpCategory::Infinite;
use std::f32::INFINITY;
use crate::math::float_cmp::{max, min};

pub struct Cuboid
{
    pub m_vec0: Vector3<f32>,
    pub m_vec1: Vector3<f32>,
    pub m_color: Colorf,
    pub m_material: Option<Arc<dyn Material>>,
}

pub enum Face
{
    SMALL_X,
    BIG_X,
    SMALL_Y,
    BIG_Y,
    SMALL_Z,
    BIG_Z,
}

impl Cuboid
{
    pub fn new(vec0: Vector3<f32>, vec1: Vector3<f32>, color: Colorf) -> Cuboid
    {
        Cuboid
        {
            m_vec0: vec0,
            m_vec1: vec1,
            m_color: color,
            m_material: None
        }
    }

    pub fn get_normal(&self, face: Face) -> Vector3<f32>
    {
        match face
        {
            Face::SMALL_X => Vector3::unit_x().mul_element_wise(-1.0),
            Face::BIG_X => Vector3::unit_x(),
            Face::SMALL_Y => Vector3::unit_y().mul_element_wise(-1.0),
            Face::BIG_Y => Vector3::unit_y(),
            Face::SMALL_Z => Vector3::unit_z().mul_element_wise(-1.0),
            _ => Vector3::unit_z(),
        }
    }

    pub fn calculate_hit_time(&self, incomeray: &Ray, TMIN: &mut f32, TMAX: &mut f32) -> (bool, Face, Face)
    {
        let mut t_min = Vector3::zero();
        let mut t_max = Vector3::zero();
        let INV_VEL = Vector3::new(1.0, 1.0, 1.0).div_element_wise(incomeray.m_direction);

        if INV_VEL.x >= 0.0
        {
            t_min.x = (self.m_vec0.x - incomeray.m_origin.x) * INV_VEL.x;
            t_max.x = (self.m_vec1.x - incomeray.m_origin.x) * INV_VEL.x;
        } else {
            t_min.x = (self.m_vec1.x - incomeray.m_origin.x) * INV_VEL.x;
            t_max.x = (self.m_vec0.x - incomeray.m_origin.x) * INV_VEL.x;
        }

        if INV_VEL.y >= 0.0
        {
            t_min.y = (self.m_vec0.y - incomeray.m_origin.y) * INV_VEL.y;
            t_max.y = (self.m_vec1.y - incomeray.m_origin.y) * INV_VEL.y;
        } else {
            t_min.y = (self.m_vec1.y - incomeray.m_origin.y) * INV_VEL.y;
            t_max.y = (self.m_vec0.y - incomeray.m_origin.y) * INV_VEL.y;
        }

        if INV_VEL.z >= 0.0
        {
            t_min.z = (self.m_vec0.z - incomeray.m_origin.z) * INV_VEL.z;
            t_max.z = (self.m_vec1.z - incomeray.m_origin.z) * INV_VEL.z;
        } else {
            t_min.z = (self.m_vec1.z - incomeray.m_origin.z) * INV_VEL.z;
            t_max.z = (self.m_vec0.z - incomeray.m_origin.z) * INV_VEL.z;
        }

        let mut max_tmin = 0.0;
        let mut min_tmax = 0.0;
        let mut face_in: Face = Face::SMALL_X;
        let mut face_out: Face = Face::SMALL_X;

        let mut t_min_max_component = max(t_min.x, max(t_min.y , t_min.z ));

        if t_min_max_component == t_min.x
        {
            max_tmin = t_min.x;
            face_in = if INV_VEL.x >= 0.0 { Face::SMALL_X } else { Face::BIG_X };
        }
        else if t_min_max_component == t_min.y
        {
            max_tmin = t_min.y;
            face_in = if INV_VEL.y >= 0.0 { Face::SMALL_Y } else { Face::BIG_Y };
        }
        else
        {
            max_tmin = t_min.z;
            face_in = if INV_VEL.z >= 0.0 { Face::SMALL_Z } else { Face::BIG_Z };
        }

        let mut t_max_min_component = min( t_max.x , min(t_max.y, t_max.z));

        if t_max_min_component == t_max.x
        {
            min_tmax = t_max.x;
            face_out = if INV_VEL.x >= 0.0 { Face::SMALL_X } else { Face::BIG_X };
        }
        else if t_max_min_component == t_max.y
        {
            min_tmax = t_max.y;
            face_out = if INV_VEL.y >= 0.0 { Face::SMALL_Y } else { Face::BIG_Y };
        }
        else
        {
            min_tmax = t_max.z;
            face_out = if INV_VEL.z >= 0.0 { Face::SMALL_Z } else { Face::BIG_Z };
        }
        if max_tmin < min_tmax
        {
            *TMIN = max_tmin;
            *TMAX = min_tmax;
            return (true, face_in, face_out);
        }
        return (false, face_in, face_out);
    }
}

impl fmt::Debug for Cuboid
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Cuboid")
            .field("Vector 0", &self.m_vec0)
            .field("Vector 1", &self.m_vec1)
            .finish()
    }
}

impl Geometry for Cuboid
{
    unsafe fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        let mut TMIN = INFINITY;
        let mut TMAX = 0.0_f32;

        if let (true, face_in, face_out) = self.calculate_hit_time(incomeray, &mut TMIN, &mut TMAX)
        {
            if TMIN > KEPSILON && *time > TMIN
            {
                *time = TMIN;
                shaderecord.m_normal = self.get_normal(face_in);
            }
            else if *time > TMAX
            {
                shaderecord.m_normal = self.get_normal(face_out);
            }
            shaderecord.m_hitpoint = incomeray.m_origin + *time * incomeray.m_direction;
            return Ok(true)
        }
        Ok(false)
    }
}

impl Boundable for Cuboid
{
    fn get_bbox(&self) -> BBox {
        BBox::new(self.m_vec0, self.m_vec1 )
    }
}

impl Shadable for Cuboid
{
    fn get_color(&self) -> Colorf
    {
        self.m_color
    }

    fn set_color(&mut self, newcolor: Colorf)
    {
        self.m_color = newcolor;
    }

    fn get_material(&self) -> Arc<dyn Material>
    {
        if let Some(x) = self.m_material.clone() { x }
        else { panic!("The material for Cuboid is Not set") }
    }

    fn set_material(&mut self, material: Arc<dyn Material>) {
        self.m_material = Some(material.clone());
    }

    fn shadow_hit(&self, ray: &Ray, tmin: &mut f32) -> bool {
        true
    }
}

#[cfg(test)]
mod CuboidTest
{
    use cgmath::Vector3;
    use std::f32::INFINITY;
    use approx::{assert_relative_eq};

    use super::*;
    use crate::utils::colorconstant::COLOR_BLACK;
    use crate::world::viewplane::ViewPlane;
    use crate::tracer::whitted::Whitted;
    use crate::world::world::World;
    use crate::sampler::mutijittered::MultiJittered;

    #[test]
    fn check_hit_small_x()
    {
        let v0 = Vector3::new(0.0, -5.0, 6.0);
        let v1 = Vector3::new(5.0, 0.0, 10.0);
        let cuboid = Cuboid::new(v0, v1, COLOR_BLACK);

        let mut sampler = MultiJittered::new(256, 1);
        let vp = Box::new(ViewPlane::new(Arc::new(sampler)));
        let mut sr = ShadeRec::new(Arc::new(World::new(vp)));

        let ray = Ray::new(Vector3::new(-10.0, -2.0, 8.0),
                           Vector3::new(1.0, 0.0, 0.0));
        let mut t = INFINITY;
        let res = cuboid.hit(&ray, &mut t,&mut sr);

        assert_eq!(res.unwrap(), true);
        assert_relative_eq!(sr.m_normal, -Vector3::unit_x());
        assert_relative_eq!(t, 10.0);
    }
}