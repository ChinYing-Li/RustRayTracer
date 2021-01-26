use cgmath::{Vector2, Vector3};

use crate::sampler::Sampler;

#[derive(Clone, Debug)]
pub struct DummySampler
{}

impl DummySampler
{
    pub fn new(sample_per_pattern: usize, num_pattern: usize) -> DummySampler
    {
        DummySampler {}
    }
}

impl Sampler for DummySampler
{
    fn generate_sample_pattern(&mut self) {
        unimplemented!()
    }

    fn set_map_to_disk(&mut self, flag: bool) {
        unimplemented!()
    }

    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32) {
        unimplemented!()
    }

    fn get_unit_square_pattern(&mut self) -> &Vec<Vector2<f32>>
    {
        unimplemented!()
    }

    fn get_disk_pattern(&mut self) -> &Vec<Vector2<f32>> {
        unimplemented!()
    }

    fn get_disk_sample(&mut self) -> Vector2<f32> {
        unimplemented!()
    }

    fn get_hemisphere_pattern(&mut self) -> &Vec<Vector3<f32>> {
        unimplemented!()
    }

    fn get_hemisphere_sample(&mut self) -> Vector3<f32> {
        unimplemented!()
    }
}