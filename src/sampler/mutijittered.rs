use crate::sampler::{SamplerCore, Sampler};
use cgmath::{Vector2, Vector3, ElementWise};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use std::cell::RefCell;
use std::ops::Deref;
use rand::prelude::StdRng;

#[derive(Debug)]
pub struct MultiJittered
{
    m_core: SamplerCore,
}

impl MultiJittered
{
    pub fn new(sample_per_pattern: usize, num_pattern: usize) -> MultiJittered
    {
        let actual_sample_per_pattern = (sample_per_pattern as f32).sqrt().floor().powf(2.0);
        let core = SamplerCore::new(actual_sample_per_pattern as usize, num_pattern);

        MultiJittered
        {
            m_core: core,
        }
    }
}

impl Sampler for MultiJittered
{
    fn generate_sample_pattern(&mut self)
    {
        let sqrt_samples_per_pattern = (self.m_core.m_sample_per_pattern as f32).sqrt() as usize;
        let inv_sqrt = 1.0 / sqrt_samples_per_pattern as f32;
        let mut rng = thread_rng();
        print!("sample patter, {}", self.m_core.m_samples_on_square.len());

        for pattern in 0..self.m_core.m_num_pattern
        {
            for j in 0..sqrt_samples_per_pattern
            {
                for i in 0..sqrt_samples_per_pattern
                {
                    self.m_core.m_samples_on_square[pattern][i * sqrt_samples_per_pattern + j] =
                        (Vector2::new((j as f32 + rng.gen_range(0.0, 1.0)) * inv_sqrt.powf(2.0),
                                      (i as f32 + rng.gen_range(0.0, 1.0)) * inv_sqrt.powf(2.0)));
                }
            }
        }

        self.m_core.shuffle_x_coordinates();
        self.m_core.shuffle_y_coordinates();

        for p in 0..self.m_core.m_num_pattern
        {
            for row in 0..sqrt_samples_per_pattern
            {
                for i in 0..sqrt_samples_per_pattern
                {
                    self.m_core.m_samples_on_square[p][(row * sqrt_samples_per_pattern + i) as usize]
                        .add_assign_element_wise(Vector2::new(row as f32 * inv_sqrt, i as f32 * inv_sqrt));
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

    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32)
    {
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
            _ =>
                {
                    panic!("The MultiJittered Sampler isn't set to generate samples on hemisphere");
                }
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
            _ => panic!("The MultiJittered Sampler isn't set to generate samples on hemisphere")
        }
    }

    fn get_hemisphere_sample(&self) -> Vector3<f32>
    {
        self.m_core.get_hemisphere_sample()
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

    const INV_PI: f32 = 1.0 / PI;
    const INV_GAMMA: f32 = 1.0 / 1.8;
    // This is not proper unit testing. We just write the result to
    // image to inspect whether the sampler is implemented correctly.
    #[test]
    pub fn DrawSquareSamples()
    {
        let width = 256;
        let height = 256;
        let imgname = "test/output/MultiJittered_square.jpg";
        let mut imgwriter = ImageWriter::new(imgname, width, height);

        let mut sampler = MultiJittered::new(32, 2);
        sampler.generate_sample_pattern();

        for sample in sampler.m_core.m_samples_on_square.iter()
        {
            let x = sample.x * (width as f32);
            let y = sample.y * (height as f32);
            imgwriter.write_pixel(min(x, width -1), min(y, height -1), COLOR_WHITE, INV_GAMMA as f32);
        }
        assert_eq!(sampler.get_sample_per_pattern(), 25);
        imgwriter.output();
    }

    #[test]
    pub fn DrawDiskSamples()
    {
        let width = 256;
        let height = 256;
        let imgname = "test/output/MultiJittered_Disk.jpg";
        let mut imgwriter = ImageWriter::new(imgname, width, height);

        let mut sampler = MultiJittered::new(144, 3);
        sampler.set_map_to_disk(true);
        sampler.generate_sample_pattern();

        let radius = (min(width, height) / 2 - 2 ) as f32;
        for sample in sampler.m_core.m_samples_on_disk.unwrap().iter()
        {
            println!("sample on disk");
            let x = sample.x * radius + radius;
            let y = sample.y * radius + radius;
            imgwriter.write_pixel(min(x, width-1), min(y, height-1), COLOR_WHITE, INV_GAMMA as f32);
        }

        for theta in 0..360
        {
            imgwriter.write_pixel((radius * (theta as f32 * INV_PI).cos() + radius) as usize,
                                  (radius * (theta as f32 * INV_PI).sin() + radius) as usize,
                                    COLOR_GREEN, INV_GAMMA as f32);
        }
        imgwriter.output();
    }

    #[test]
    pub fn DrawHemisphereSamples()
    {
        let width = 256;
        let height = 256;
        let imgname = "test/output/MultiJittered_Hemisphere.jpg";
        let mut imgwriter = ImageWriter::new(imgname, width, height);

        let mut sampler = MultiJittered::new(256, 2);
        sampler.set_map_to_hemisphere(true, 1.0);
        sampler.generate_sample_pattern();

        let radius = (min(width, height) / 2 - 2 ) as f32;
        for pattern in sampler.m_core.m_samples_on_hemisphere.iter()
        {
            for sample in pattern.iter()
            {
                println!("sample on hemisphere");
                let x = sample.x * radius + radius;
                let y = sample.y * radius + radius;
                imgwriter.write_pixel(x as usize, y as usize, COLOR_WHITE, INV_GAMMA as f32);
            }
        }

        for theta in 0..360
        {
            imgwriter.write_pixel((radius * (theta as f32 * INV_PI).cos() + radius) as usize,
                                  (radius * (theta as f32 * INV_PI).sin() + radius) as usize,
                                  COLOR_GREEN, INV_GAMMA as f32);
        }
        imgwriter.output();
    }
}