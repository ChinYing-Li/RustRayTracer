use crate::geometry::{Geometry, GeomError, Boundable, BoundedConcrete};
use std::sync::Arc;
use cgmath::{Matrix3, SquareMatrix, Vector3, Matrix4, InnerSpace, ElementWise, Rad, Deg, Zero, Vector4};
use crate::world::shaderec::ShadeRec;
use crate::ray::Ray;
use std::fmt;
use std::ptr::null;
use crate::math::constants;
use crate::material::Material;
use crate::geometry::bbox::BBox;
use std::f32::INFINITY;

#[derive(Clone)]
pub struct Instance
{
    m_geomptr: Arc<dyn BoundedConcrete>,
    m_material_ptr: Arc<dyn Material>,
    m_inv_matrix: Matrix4<f32>,
    m_forward_matrix: Matrix4<f32>,
    m_do_transform_texture: bool,
    m_bbox: BBox,
}

impl Instance
{
    pub fn new(geomptr: Arc<dyn BoundedConcrete>) -> Instance
    {
        let mut mat_buffer = Matrix4::identity();
        mat_buffer[3][3] = 0.0;

        Instance
        {
            m_geomptr: geomptr.clone(),
            m_material_ptr: geomptr.get_material(),
            m_inv_matrix: mat_buffer,
            m_forward_matrix: mat_buffer,
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

    pub fn set_material(&mut self, material: Arc<dyn Material>)
    {
        self.m_material_ptr = material;
    }

    pub fn translate(&mut self, displace: Vector3<f32>)
    {
        // the last element of displace must be zero
        let augmented_displace = Vector4::new(displace.x, displace.y, displace.z, 0.0);
        self.m_inv_matrix[3] -= augmented_displace;
        self.m_forward_matrix[3] += augmented_displace;
        self.compute_bbox();
    }

    // theta is in radian
    pub fn rotate_x(&mut self, theta: f32)
    {
        self.m_forward_matrix = Matrix4::from_angle_x(Deg(theta)) * self.m_forward_matrix;
        self.m_inv_matrix = Matrix4::from_angle_x(-Deg(theta)) * self.m_forward_matrix;
        self.compute_bbox();
    }

    pub fn rotate_y(&mut self, theta: f32)
    {
        self.m_forward_matrix = Matrix4::from_angle_y(Deg(theta)) * self.m_forward_matrix;
        self.m_inv_matrix = Matrix4::from_angle_y(-Deg(theta)) * self.m_forward_matrix;
        self.compute_bbox();
    }

    pub fn rotate_z(&mut self, theta: f32)
    {
        self.m_forward_matrix = Matrix4::from_angle_z(Deg(theta)) * self.m_forward_matrix;
        self.m_inv_matrix = Matrix4::from_angle_z(-Deg(theta)) * self.m_forward_matrix;
        self.compute_bbox();
    }

    fn transform_vector3(mat4: &Matrix4<f32>, vector3: &Vector3<f32>) -> Vector3<f32>
    {
        let vector4 = Vector4::new(vector3.x, vector3.y, vector3.z, 1.0);
        let transformed_vector4 = (*mat4) * vector4;
        Vector3::new(transformed_vector4.x, transformed_vector4.y, transformed_vector4.z)
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
        let mut inverted_ray = Ray::new(Instance::transform_vector3(&self.m_inv_matrix, &(incomeray.m_origin)),
                                        Instance::transform_vector3(&self.m_inv_matrix, &incomeray.m_direction));

        if self.m_geomptr.hit(&inverted_ray, time, shaderecord)
            .unwrap_or(false)
        {
            shaderecord.m_normal = Instance::transform_vector3(&self.m_inv_matrix, &shaderecord.m_normal);
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
        let mut trans_vertices = vec![self.m_bbox.m_vertex_0,
            Vector3::new(self.m_bbox.m_vertex_1.x,
                         self.m_bbox.m_vertex_0.y,
                         self.m_bbox.m_vertex_0.z),
            Vector3::new(self.m_bbox.m_vertex_0.x,
                         self.m_bbox.m_vertex_1.y,
                         self.m_bbox.m_vertex_0.z),
            Vector3::new(self.m_bbox.m_vertex_0.x,
                         self.m_bbox.m_vertex_0.y,
                         self.m_bbox.m_vertex_1.z),
            Vector3::new(self.m_bbox.m_vertex_1.x,
                         self.m_bbox.m_vertex_1.y,
                         self.m_bbox.m_vertex_0.z),
            Vector3::new(self.m_bbox.m_vertex_1.x,
                         self.m_bbox.m_vertex_0.y,
                         self.m_bbox.m_vertex_1.z),
            Vector3::new(self.m_bbox.m_vertex_0.x,
                         self.m_bbox.m_vertex_1.y,
                         self.m_bbox.m_vertex_1.z),
            self.m_bbox.m_vertex_1];

        trans_vertices = trans_vertices.iter()
            .map(|vertex| Instance::transform_vector3(&self.m_forward_matrix, vertex))
            .collect();

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

#[cfg(test)]
mod InstanceTest
{
    use super::*;
    use std::f32::consts::PI;
    use crate::geometry::sphere::Sphere;
    use approx::{assert_relative_eq};
    use crate::utils::colorconstant::COLOR_RED;
    use crate::material::matte::Matte;
    use crate::brdf::lambertian::Lambertian;
    use crate::utils::color::Colorf;
    use crate::geometry::Shadable;
    use crate::geometry::triangle::Triangle;

    const INV_PI: f32 = 1.0 / PI ;
    const INV_GAMMA: f32 = 1.0 / 1.8;

    #[test]
    pub fn TestTranslate()
    {
        let sphere = setup_sphere();
        let mut instance = Instance::new(Arc::new(sphere));
        instance.translate(Vector3::new(-5.0, 10.0,10.0));
        let instance_bbox = instance.get_bbox();

        assert_relative_eq!(instance_bbox.m_vertex_0.x, 5.85786437626 - 5.0);
        assert_relative_eq!(instance_bbox.m_vertex_0.y, -4.1421356237 + 10.0);
        assert_relative_eq!(instance_bbox.m_vertex_1.z, 44.1421356237 + 10.0);

    }

    #[test]
    fn TestRotateX()
    {
        let triangle = setup_triangle();
    }

    #[test]
    fn TestRotateY()
    {
        let triangle = setup_triangle();
    }

    #[test]
    fn TestRotateZ()
    {
        let triangle = setup_triangle();
    }

    fn setup_sphere() -> Sphere
    {
        let mut sphere = Sphere::new(10.0, Vector3::new(20.0, 10.0, 30.0));
        let material = Matte::new(Arc::new(Lambertian::new(0.5, Colorf::new(0.3, 0.3, 0.3))),
                   Arc::new(Lambertian::new(0.3, Colorf::new(0.5, 0.2, 0.5))));
        sphere.set_material(Arc::new(material));
        sphere
    }

    fn setup_triangle() -> Triangle
    {
        let v0 = Vector3::new(1.0, 0.4, -1.0);
        let v1 = Vector3::new(2.0, 1.0, -10.0);
        let v2 = Vector3::new(0.5, 9.0, 1.0);
        Triangle::new(v0, v1, v2)
    }
}