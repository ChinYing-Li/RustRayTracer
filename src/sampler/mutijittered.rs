use crate::sampler::{SamplerCore, Sampler};
use cgmath::{Vector2, Vector3, ElementWise};
use rand::Rng;

pub struct MultiJittered
{
    m_core: SamplerCore
}

impl MultiJittered
{
    pub fn new(sample_per_pattern: usize, num_pattern: usize) -> MultiJittered
    {
        let core = SamplerCore::new(sample_per_pattern, num_pattern);

        MultiJittered
        {
            m_core: core,
        }
    }
}

impl Sampler for MultiJittered
{
    fn generate_sample_pattern(&mut self) {
        self.m_core.m_samples.clear();

        let sqrt_samples_per_pattern = (self.m_core.m_sample_per_pattern as f32).sqrt().floor() as u16;
        let inv_sqrt = 1.0 / sqrt_samples_per_pattern as f32;

        let mut rng = rand::thread_rng();

        for row in 0..sqrt_samples_per_pattern
        {
            for i in 0..sqrt_samples_per_pattern
            {
                self.m_core.m_samples.push(Vector2::new((row as f32 + rng.gen_range(0.0, 1.0)) * inv_sqrt.powf(2.0),
                                                        (i as f32 + rng.gen_range(0.0, 1.0)) * inv_sqrt.powf(2.0)));
            }
        }

        self.m_core.shuffle_x_coordinates();
        self.m_core.shuffle_y_coordinates();

        for row in 0..sqrt_samples_per_pattern
        {
            for i in 0..sqrt_samples_per_pattern
            {
                self.m_core.m_samples[(row * sqrt_samples_per_pattern + i) as usize].add_assign_element_wise(Vector2::new(row as f32 * inv_sqrt, i as f32 * inv_sqrt));
            }
        }
    }

    fn set_map_to_disk(&mut self, flag: bool) {
        unimplemented!()
    }

    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32) {
        unimplemented!()
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
mod MultiJitteredTest
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
        let imgname = "test/output/MultiJittered.jpg";
        let mut imgwriter = ImageWriter::new(imgname, width as u16, height as u16);

        let mut sampler = MultiJittered::new(16, 1);
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