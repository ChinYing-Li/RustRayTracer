use cgmath::Vector2;
use crate::sampler::{Sampler, SamplerCore};
use rand::Rng;

pub struct Jittered
{
    m_core: SamplerCore,
    m_indices_for_shuffling: Vec<u16>,
    m_index_step: u16,
}

impl Sampler for Jittered
{
    fn generateSamplePattern(&mut self)
    {
        let sqrt_sample_per_pattern = (self.m_core.m_sample_per_pattern as f32).sqrt() as u16;
        let inv_sqrt = 1.0 / sqrt_sample_per_pattern as f32;
        let mut rng = rand::thread_rng();

        for _ in 0..self.m_core.m_num_pattern
        {
            for i in 0..sqrt_sample_per_pattern
            {
                for j in 0..sqrt_sample_per_pattern
                {
                    self.m_core.m_samples.push(Vector2::new(i as f32 * inv_sqrt + rng.gen_range(0.0, 1.0),
                                                                  j as f32* inv_sqrt + rng.gen_range(0.0, 1.0)));
                }
            }
        }
    }

    fn shuffleIndices(&mut self)
    {

    }

    fn setupShuffleIndices(&mut self)
    {

    }
}