use cgmath::{Vector2, Vector3};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::borrow::BorrowMut;
use std::cell::RefCell;

use crate::sampler::{Sampler, SamplerCore};

#[derive(Clone, Debug)]
pub struct Jittered
{
    m_core: SamplerCore,
    m_rng: RefCell<ThreadRng>,
}

impl Jittered
{
    pub fn new(sample_per_pattern: usize, num_pattern: usize) -> Jittered
    {
        let core = SamplerCore::new(sample_per_pattern, num_pattern);

        Jittered
        {
            m_core: core,
            m_rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl Sampler for Jittered
{
    fn generate_sample_pattern(&mut self)
    {
        let sqrt_sample_per_pattern = (self.m_core.m_sample_per_pattern as f32).sqrt() as usize;
        let inv_sqrt = 1.0 / sqrt_sample_per_pattern as f32;
        let mut rng_ref = self.m_rng.borrow_mut();

        for pattern in 0..self.m_core.m_num_pattern
        {
            for i in 0..sqrt_sample_per_pattern
            {
                for j in 0..sqrt_sample_per_pattern
                {
                    self.m_core.m_samples[pattern][i * sqrt_sample_per_pattern + j] = Vector2::new(i as f32 * inv_sqrt + rng_ref.gen_range(0.0, inv_sqrt),
                                                                  j as f32 * inv_sqrt + rng_ref.gen_range(0.0, inv_sqrt));
                }
            }
        }

        if self.m_core.m_map_to_disk { self.m_core.map_sample_to_disk(); }
        if self.m_core.m_map_to_hemisphere { self.m_core.map_sample_to_hemisphere(1.0); }
    }

    fn get_sample_per_pattern(&self) -> usize
    {
        self.m_core.m_sample_per_pattern
    }

    fn set_map_to_disk(&mut self, flag: bool) {
        self.m_core.set_map_to_disk(flag);
    }

    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32) {
        self.m_core.set_map_to_hemisphere(flag, e);
    }

    fn get_unit_square_pattern(&self) -> &Vec<Vector2<f32>>
    {
        self.m_core.get_unit_square_pattern()
    }

    fn get_disk_pattern(&self) -> &Vec<Vector2<f32>>
    {
        match self.m_core.get_disk_pattern()
        {
            Ok(sample) => sample,
            _ => panic!("The Jittered Sampler isn't set to generate samples on disk")
        }
    }

    fn get_disk_sample(&self) -> Vector2<f32>
    {
        self.m_core.get_disk_sample()
    }

    fn get_hemisphere_pattern(&self) -> &Vec<Vector3<f32>>
    {
        match self.m_core.get_hemisphere_pattern()
        {
            Ok(sample) => sample,
            _ => panic!("The Jittered Sampler isn't set to generate samples on hemisphere")
        }
    }

    fn get_hemisphere_sample(&self) -> Vector3<f32>
    {
        self.m_core.get_hemisphere_sample()
    }
}

#[cfg(test)]
mod JitteredTest
{
    use super::*;
    use crate::output::OutputManager;
    use crate::output::imagewriter::ImageWriter;
    use crate::utils::colorconstant::COLOR_WHITE;
    use std::f32::consts::PI;

    const INV_PI: f32 = 1.0 / PI ;
    const INV_GAMMA: f32 = 1.0 / 1.8;

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
            imgwriter.write_pixel(x as u16, y as u16, COLOR_WHITE, INV_GAMMA);
        }

        imgwriter.output();
    }
}