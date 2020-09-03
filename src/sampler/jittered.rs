use cgmath::{Vector2, Vector3};
use crate::sampler::{Sampler, SamplerCore};
use rand::Rng;

pub struct Jittered
{
    m_core: SamplerCore,
    m_indices_for_shuffling: Vec<u16>,
    m_index_step: u16,
}

impl Jittered
{
    pub fn new(sample_per_pattern: usize, num_pattern: usize) -> Jittered
    {
        let core = SamplerCore::new(sample_per_pattern, num_pattern);

        Jittered
        {
            m_core: core,
            m_indices_for_shuffling: Vec::with_capacity(0),
            m_index_step: 5, // TODO refactor Jittered
        }
    }
}

impl Sampler for Jittered
{
    fn generate_sample_pattern(&mut self)
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
                    self.m_core.m_samples.push(Vector2::new(i as f32 * inv_sqrt + rng.gen_range(0.0, inv_sqrt),
                                                                  j as f32 * inv_sqrt + rng.gen_range(0.0, inv_sqrt)));
                }
            }
        }
    }

    fn set_map_to_disk(&mut self, flag: bool) {
        self.m_core.set_map_to_disk(flag);
    }

    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32) {
        self.m_core.set_map_to_hemisphere(flag, e);
    }

    fn get_unit_square_sample(&self) -> Vector2<f32> {
        unimplemented!()
    }

    fn get_disk_sample(&self) -> Vector2<f32> {
        unimplemented!()
    }

    fn get_hemisphere_sample(&self) -> Vector3<f32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod JitteredTest
{
    use super::*;
    use crate::output::OutputManager;
    use crate::output::imagewriter::ImageWriter;
    use crate::utils::colorconstant::COLOR_WHITE;

    // This is not proper unit testing. We just write the result to
    // image to inspect whether the sampler is implemented correctly.
    #[test]
    pub fn DrawResult()
    {
        let width = 256;
        let height = 256;
        let imgname = "test/output/Jittered.jpg";
        let mut imgwriter = ImageWriter::new(imgname, width as u16, height as u16);

        let mut sampler = Jittered::new(16, 1);
        sampler.generate_sample_pattern();

        for sample in sampler.m_core.m_samples.iter()
        {
            let x = sample.x * (width as f32);
            let y = sample.y * (height as f32);
            imgwriter.write_pixel(x as u16, y as u16, COLOR_WHITE);
        }

        imgwriter.output();
    }
}