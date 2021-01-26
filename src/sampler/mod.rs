pub mod mutijittered;
pub mod nrooks;
pub mod jittered;
pub mod dummy;

use cgmath::{Vector2, Vector3, ElementWise, Zero};
use std::{f32};
use cgmath::num_traits::Inv;
use rand::{Rng, seq::SliceRandom};
use std::error::Error;
use std::fmt;

type Point2<T> = Vector2<T>;

#[derive(Clone, Debug)]
struct SamplerCore
{
    pub m_sample_per_pattern: usize,
    pub m_num_pattern: usize,
    m_map_to_disk: bool,
    m_map_to_hemisphere: bool,
    m_samples_on_square: Vec<Vec<Vector2<f32>>>,
    m_samples_on_disk: Vec<Vec<Vector2<f32>>>,
    m_samples_on_hemisphere: Vec<Vec<Vector3<f32>>>,

    pub m_shuffled_indices: Vec<u32>,
}

impl SamplerCore
{
    fn new(sample_per_pattern: usize, num_pattern: usize) -> SamplerCore
    {
        SamplerCore
        {
            m_sample_per_pattern: sample_per_pattern,
            m_num_pattern: num_pattern,
            m_samples_on_square: vec![vec![Vector2::zero(); sample_per_pattern]; num_pattern],
            m_map_to_disk: false,
            m_map_to_hemisphere: false,
            m_samples_on_disk: Vec::with_capacity(num_pattern),
            m_samples_on_hemisphere: Vec::with_capacity(num_pattern),

            m_shuffled_indices: SamplerCore::setup_shuffled_indices(num_pattern, sample_per_pattern),
            m_rng: StdRng::new(),
        }
    }

    // TODO: What the heck is this "flag" variable???
    fn set_map_to_disk(&mut self, flag: bool)
    {
        if flag != self.m_map_to_disk
        {
            self.m_map_to_disk = flag;
            if flag && !self.m_samples_on_square.is_empty()
            {
                self.map_sample_to_disk();
            }
        }
    }

    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32)
    {
        if flag != self.m_map_to_hemisphere
        {
            self.m_map_to_hemisphere = flag;
            if flag && !self.m_samples_on_square.is_empty()
            {
                self.map_sample_to_hemisphere(e);
            }
        }
    }

    fn get_unit_square_pattern(&mut self) -> &Vec<Vector2<f32>>
    {
        let pattern_index = self.get_pattern_index();
        &self.m_samples_on_square[pattern_index]
    }

    fn get_disk_pattern(&mut self) -> Result<&Vec<Vector2<f32>>, &str>
    {
        if !self.m_map_to_disk { return Err("Didn't yet generate disk samples") }
        let pattern_index = self.get_pattern_index();
        Ok(&self.m_samples_on_disk[pattern_index])
    }

    fn get_disk_sample(&mut self) -> Vector2<f32>
    {
        if self.m_samples_on_disk.len() != 0
        {
            let pattern_index = self.get_pattern_index();
            let sample_index = self.m_rng.gen::<usize>() % self.m_sample_per_pattern;
            self.m_samples_on_disk[pattern_index][sample_index]
        }
        else { panic!("Didn't yet generate hemisphere samples") }
    }

    fn get_hemisphere_pattern(&mut self) -> Result<&Vec<Vector3<f32>>, &str>
    {
        if !self.m_map_to_hemisphere { return Err("") }
        let pattern_index = self.get_pattern_index();
        Ok(&self.m_samples_on_hemisphere[pattern_index])
    }

    fn get_hemisphere_sample(&mut self) -> Vector3<f32>
    {
        if self.m_samples_on_hemisphere.len() != 0
        {
            let pattern_index = self.get_pattern_index();
            let sample_index = self.m_rng.gen::<usize>() % self.m_sample_per_pattern;
            self.m_samples_on_hemisphere[pattern_index][sample_index]
        }
        else { panic!("Didn't yet generate hemisphere samples") }
    }

    fn shuffle_x_coordinates(&mut self)
    {
        assert_eq!(self.m_samples_on_square.len(), self.m_num_pattern);
        assert_eq!(self.m_samples_on_square[0].len(), self.m_sample_per_pattern);
        let mut rng = rand::thread_rng();

        for i in 0..self.m_num_pattern
        {
            for j in 0..self.m_sample_per_pattern
            {
                let target_sample_index = rng.gen::<usize>() % self.m_sample_per_pattern;
                let temp = self.m_samples_on_square[i][j].x;
                self.m_samples_on_square[i][j].x = self.m_samples_on_square[i][target_sample_index].x;
                self.m_samples_on_square[i][target_sample_index].x = temp;
                /*
                let target = rng.gen::<usize>() % self.m_sample_per_pattern + i * self.m_sample_per_pattern;
                let temp = self.m_samples[j + i * self.m_sample_per_pattern].x;
                self.m_samples[j + i * self.m_sample_per_pattern].x = self.m_samples[target].x;
                self.m_samples[target].x = temp;
                 */
            }
        }
    }

    fn shuffle_y_coordinates(&mut self)
    {
        assert_eq!(self.m_samples_on_square.len(), self.m_num_pattern);
        assert_eq!(self.m_samples_on_square[0].len(), self.m_sample_per_pattern);

        for i in 0..self.m_num_pattern
        {
            for j in 0..self.m_sample_per_pattern
            {
                let target_sample_index = self.m_rng.gen::<usize>() % self.m_sample_per_pattern;
                let temp = self.m_samples_on_square[i][j].y;
                self.m_samples_on_square[i][j].y = self.m_samples_on_square[i][target_sample_index].y;
                self.m_samples_on_square[i][target_sample_index].y = temp;
                /*
                let temp = self.m_samples[j + i * self.m_sample_per_pattern].y;
                self.m_samples[j + i * self.m_sample_per_pattern].y = self.m_samples[target].y;
                self.m_samples[target].y = temp;
                */
            }
        }
    }

    fn setup_shuffled_indices(num_pattern: usize, sample_per_pattern: usize) -> Vec<u32>
    {
        let mut rng = rand::thread_rng();
        let mut shuffled_indices: Vec<u32> = Vec::with_capacity(sample_per_pattern * num_pattern);
        let mut orig_indices: Vec<_> = (0..sample_per_pattern as u32).collect();

        for _ in 0..num_pattern
        {
            orig_indices.shuffle(&mut rng);
            shuffled_indices.extend(&orig_indices);
        }
        shuffled_indices
    }

    fn map_sample_to_disk(&mut self)
    {
        if self.m_samples_on_disk.len() != 0
        { return; }
        let mut radius = 0.0;
        let mut phi = 0.0;
        let mut samp = Vector2::new(0.0, 0.0);

        let n = self.m_num_pattern * self.m_sample_per_pattern;
        let mut diskpattern = Vec::with_capacity(n);
        let identityvec = Vector2::new(1.0, 1.0);

        for pattern in self.m_samples_on_square.iter()
        {
            for square_sample in pattern.iter()
            {
                samp = square_sample.mul_element_wise(2.0) - identityvec;
                if samp.x > -samp.y
                {
                    if samp.x > samp.y
                    {
                        radius = samp.x;
                        phi = samp.y / samp.x;
                    }
                    else
                    {
                        radius = samp.y;
                        phi = 2.0 - samp.x / samp.y;
                    }
                }
                else
                {
                    if samp.x < samp.y
                    {
                        radius = - samp.x;
                        phi = 4.0 + samp.y / samp.x;
                    }
                    else {
                        radius = - samp.y;
                        if samp.y != 0.0
                        {
                            phi = 6.0 - samp.x / samp.y;
                        }
                        else { phi = 0.0; }
                    }
                }

                phi *= f32::consts::PI / 4.0;
                diskpattern.push(Vector2::new(radius * phi.cos(), radius * phi.sin()));
            }
        }

        self.m_samples_on_disk = (0..self.m_num_pattern)
            .map(|pattern_index| diskpattern[pattern_index * self.m_sample_per_pattern..(pattern_index+1) * self.m_sample_per_pattern].to_vec())
            .collect();
    }

    fn map_sample_to_hemisphere(&mut self, e: f32)
    {
        let n = self.m_num_pattern * self.m_sample_per_pattern;
        let mut hemisphere_pattern = Vec::with_capacity(n);
        for pattern in self.m_samples_on_square.iter()
        {
            for s in pattern.iter()
            {
                let cos_phi = (2.0 * f32::consts::PI * s.x).cos();
                let sin_phi = (2.0 * f32::consts::PI * s.x).sin();
                let cos_theta = (1.0 - s.y).powf((e + 1.0).inv());
                let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();
                let pu = sin_theta * cos_phi;
                let pv = sin_theta * sin_phi;
                let pw = cos_theta;
                hemisphere_pattern.push(Vector3::new(pu, pv, pw));
            }
        }
        self.m_samples_on_hemisphere = (0..self.m_num_pattern)
            .map(|pattern_index| hemisphere_pattern[pattern_index * self.m_sample_per_pattern..(pattern_index+1) * self.m_sample_per_pattern].to_vec())
            .collect();
    }

    fn get_pattern_index(&mut self) -> usize
    {
        self.m_rng.gen::<usize>() % self.m_num_pattern
    }
}

pub trait Sampler: Send + Sync
{
    fn generate_sample_pattern(&mut self);
    fn get_sample_per_pattern(&mut self) -> usize { 1 }
    fn set_map_to_disk(&mut self, flag: bool);
    fn set_map_to_hemisphere(&mut self, flag: bool, e: f32);
    fn get_unit_square_pattern(&mut self) -> &Vec<Vector2<f32>>;
    fn get_disk_pattern(&mut self) -> &Vec<Vector2<f32>>;
    fn get_disk_sample(&mut self) -> Vector2<f32>;
    fn get_hemisphere_pattern(&mut self) -> &Vec<Vector3<f32>>;
    fn get_hemisphere_sample(&mut self) -> Vector3<f32>;
}

impl fmt::Debug for dyn Sampler
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sampler")
            .finish()
    }
}

#[cfg(test)]
mod ConvertSampleTest
{
    use super::*;
    use approx::assert_relative_eq;
    use std::f32;

    #[test]
    fn checkMap2Disk()
    {
        let mut core = SamplerCore::new(4, 2);
        core.m_samples_on_square[0].push(Vector2::new(0.8, 0.7));
        core.m_samples_on_square[0].push(Vector2::new(0.5, 0.6));
        core.m_samples_on_square[0].push(Vector2::new(0.7, 0.3));
        core.m_samples_on_square[0].push(Vector2::new(0.1, 0.2));
        core.m_samples_on_square[1].push(Vector2::new(0.5, 0.0));
        core.m_samples_on_square[1].push(Vector2::new(0.0, 0.0));
        // place holder
        core.m_samples_on_square[1].push(Vector2::new(0.0, 0.0));
        core.m_samples_on_square[1].push(Vector2::new(0.0, 0.0));

        core.map_sample_to_disk();

        assert_relative_eq!(core.m_samples_on_disk.clone().unwrap()[0].y, 0.2999999999999, epsilon = f32::EPSILON);
        assert_relative_eq!(core.m_samples_on_disk.clone().unwrap()[1].y, 0.0000000000, epsilon = f32::EPSILON);
        /*
        assert_relative_eq!(res[2].x, -0.6607183312158572, epsilon = f32::EPSILON);

        assert_relative_eq!(res[3].x, -0.07653668647301808);
        assert_relative_eq!(res[5].x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(res[5].y, 0.0, epsilon = f32::EPSILON);*/
    }
}

#[cfg(test)]
mod ShuffleTest
{
    use super::*;

    #[test]
    pub fn shuffle_indices()
    {

    }
}