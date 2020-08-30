use cgmath::Vector3;

pub struct AreaLight
{
    pub m_light_normal: Vector3<f32>,
    pub m_w_i: Vector3<f32>,
    pub m_sample_point: Vector3<f32>,

}