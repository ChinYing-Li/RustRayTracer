use crate::sampler::{SamplerCore, Sampler};
use cgmath::{Vector2, ElementWise, Vector3};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct NRooks
{
    m_core: SamplerCore,
}

impl Sampler for NRooks
{
    fn generate_sample_pattern(&mut self)
    {
        self.m_core.m_samples.clear();
        let mut rng = rand::thread_rng();

        for _ in 0..self.m_core.m_num_pattern
        {
            for i in 0..self.m_core.m_sample_per_pattern
            {
                self.m_core.m_samples.push(Vector2::new(i as f32 + rng.gen_range(0.0, 1.0),
                                                                i as f32 + rng.gen_range(0.0, 1.0))
                                                .div_element_wise(self.m_core.m_sample_per_pattern as f32));

            }
        }

        self.m_core.shuffle_x_coordinates();
        self.m_core.shuffle_y_coordinates();
    }

    fn get_sample_per_pattern(&self) -> usize
    {
        self.m_core.m_sample_per_pattern
    }

    fn set_map_to_disk(&mut self, flag: bool) {
        unimplemented!()
    }

    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32) {
        unimplemented!()
    }

    fn get_unit_square_sample(&mut self) -> Vector2<f32> {
        unimplemented!()
    }

    fn get_disk_sample(&self) -> Vector2<f32> {
        unimplemented!()
    }

    fn get_hemisphere_sample(&self) -> Vector3<f32> {
        unimplemented!()
    }
}