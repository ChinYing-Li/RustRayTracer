use crate::sampler::{SamplerCore, Sampler};
use cgmath::{Vector2, Vector3, ElementWise};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct MultiJittered
{
    m_core: SamplerCore,
    m_rng: RefCell<ThreadRng>,
}

impl MultiJittered
{
    pub fn new(sample_per_pattern: usize, num_pattern: usize) -> MultiJittered
    {
        let core = SamplerCore::new(sample_per_pattern, num_pattern);

        MultiJittered
        {
            m_core: core,
            m_rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl Sampler for MultiJittered
{
    fn generate_sample_pattern(&mut self) {
        self.m_core.m_samples.clear();

        let sqrt_samples_per_pattern = (self.m_core.m_sample_per_pattern as f32).sqrt().floor() as u16;
        let inv_sqrt = 1.0 / sqrt_samples_per_pattern as f32;
        let mut rng_ref = self.m_rng.borrow_mut();

        for row in 0..sqrt_samples_per_pattern
        {
            for i in 0..sqrt_samples_per_pattern
            {
                self.m_core.m_samples.push(Vector2::new((row as f32 + rng_ref.gen_range(0.0, 1.0)) * inv_sqrt.powf(2.0),
                                                        (i as f32 + rng_ref.gen_range(0.0, 1.0)) * inv_sqrt.powf(2.0)));
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

        if self.m_core.m_map_to_disk { self.m_core.map_sample_to_disk(); }
        if self.m_core.m_map_to_hemisphere { self.m_core.map_sample_to_hemisphere(1.0); }
    }

    fn set_map_to_disk(&mut self, flag: bool) {
        self.m_core.set_map_to_disk(flag);
    }

    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32)
    {
        self.m_core.set_map_to_hemisphere(flag, e);
    }

    fn get_unit_square_sample(&mut self) -> Vector2<f32>
    {
        self.m_core.get_unit_square_sample()
    }

    fn get_disk_sample(&self) -> Vector2<f32>
    {
        let index = self.m_rng.borrow_mut().gen::<u16>() as usize;
        match self.m_core.get_disk_sample(index)
        {
            Ok(sample) => sample,
            _ => panic!("The MultiJittered Sampler isn't set to generate samples on disk")
        }
    }

    fn get_hemisphere_sample(&self) -> Vector3<f32>
    {
        let index = self.m_rng.borrow_mut().gen::<u16>() as usize;
        match self.m_core.get_hemisphere_sample(index)
        {
            Ok(sample) => sample,
            _ => panic!("The MultiJittered Sampler isn't set to generate samples on hemisphere")
        }
    }
}

#[cfg(test)]
mod MultiJitteredTest
{
    use super::*;
    use crate::output::OutputManager;
    use crate::output::imagewriter::ImageWriter;
    use crate::utils::colorconstant::{COLOR_WHITE, COLOR_GREEN};
    use std::cmp::min;
    use std::f32::consts::PI;

    // This is not proper unit testing. We just write the result to
    // image to inspect whether the sampler is implemented correctly.
    #[test]
    pub fn DrawSquareSamples()
    {
        let width = 256;
        let height = 256;
        let imgname = "test/output/MultiJittered_square.jpg";
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

    #[test]
    pub fn DrawDiskSamples()
    {
        let width = 256;
        let height = 256;
        let imgname = "test/output/MultiJittered_Disk.jpg";
        let mut imgwriter = ImageWriter::new(imgname, width as u16, height as u16);

        let mut sampler = MultiJittered::new(256, 1);
        sampler.set_map_to_disk(true);
        sampler.generate_sample_pattern();

        let radius = (min(width, height) / 2 - 2 ) as f32;
        for sample in sampler.m_core.m_samples_on_disk.unwrap().iter()
        {
            println!("sample on disk");
            let x = sample.x * radius + radius;
            let y = sample.y * radius + radius;
            imgwriter.write_pixel(x as u16, y as u16, COLOR_WHITE);
        }

        let INV_PI = 1.0 / PI ;
        for theta in 0..360
        {
            imgwriter.write_pixel((radius * (theta as f32 * INV_PI).cos() + radius) as u16,
                                  (radius * (theta as f32 * INV_PI).sin() + radius) as u16,
                                    COLOR_GREEN);
        }
        imgwriter.output();
    }

    #[test]
    pub fn DrawHemisphereSamples()
    {
        let width = 256;
        let height = 256;
        let imgname = "test/output/MultiJittered_Hemisphere.jpg";
        let mut imgwriter = ImageWriter::new(imgname, width as u16, height as u16);

        let mut sampler = MultiJittered::new(256, 1);
        sampler.set_map_to_hemisphere(true, 1.0);
        sampler.generate_sample_pattern();

        let radius = (min(width, height) / 2 - 2 ) as f32;
        for sample in sampler.m_core.m_samples_on_hemisphere.unwrap().iter()
        {
            println!("sample on hemisphere");
            let x = sample.x * radius + radius;
            let y = sample.y * radius + radius;
            imgwriter.write_pixel(x as u16, y as u16, COLOR_WHITE);
        }

        let INV_PI = 1.0 / PI ;
        for theta in 0..360
        {
            imgwriter.write_pixel((radius * (theta as f32 * INV_PI).cos() + radius) as u16,
                                  (radius * (theta as f32 * INV_PI).sin() + radius) as u16,
                                  COLOR_GREEN);
        }
        imgwriter.output();
    }
}