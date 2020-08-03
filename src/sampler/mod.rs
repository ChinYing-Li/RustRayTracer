pub mod Jittered;

use cgmath::Vector2;
use std::{f32, f32::consts::PI};
use approx::RelativeEq;

type Point2<T> = Vector2<T>;

pub struct SamplerCore
{
    pub m_sample_per_pattern: u16,
    pub m_num_pattern: usize,
    pub m_samples: Vec<Vector2<f32>>,
}

impl SamplerCore
{
    fn new(sample_per_pattern: u16, num_pattern: usize) -> SamplerCore
    {
        SamplerCore
        {
            m_sample_per_pattern: sample_per_pattern,
            m_num_pattern: num_pattern,
            m_samples : Vec::with_capacity((sample_per_pattern as usize * num_pattern) as usize),
        }
    }
}

pub trait Sampler
{
    fn generateSamplePattern(&mut self);
    fn shuffleIndices(&mut self);
    fn setupShuffleIndices(&mut self);
}

fn MapSampleToDisk(samplecore: &mut SamplerCore) -> Vec<Vector2<f32>>
{
    let mut radius = 0.0;
    let mut phi = 0.0;
    let mut samp = Vector2::new(0.0, 0.0);
    let mut diskpattern = Vec::with_capacity(samplecore.m_num_pattern);
    let identityvec = Vector2::new(1.0, 1.0);

    for i in 0..(samplecore.m_num_pattern as u16 * samplecore.m_sample_per_pattern)
    {
        samp = 2.0 * samplecore.m_samples[i as usize] - identityvec;
        if(samp.x > -samp.y)
        {
            if(samp.x > samp.y)
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
            if (samp.x < samp.y)
            {
                radius = - samp.y;
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

        phi *= PI / 4.0;
        diskpattern[i as usize] = Vector2::new(radius*phi.cos(), radius*phi.sin());
    }
    diskpattern
}

fn mapSampleToHemisphere(core: &mut SamplerCore)
{
    // TODO: map sample from square to hemisphere
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
        let mut core = SamplerCore::new(3, 2);
        core.m_samples.push(Vector2::new(0.8, 0.7));
        core.m_samples.push(Vector2::new(0.5, 0.6));
        core.m_samples.push(Vector2::new(-0.7, 0.3));
        core.m_samples.push(Vector2::new(-0.1, -0.2));
        core.m_samples.push(Vector2::new(0.5, 0.0));
        core.m_samples.push(Vector2::new(0.0, 0.0));

        let res = MapSampleToDisk(&mut core);

        assert_relative_eq!(res[0].y, 0.5075146273309165, epsilon = f32::EPSILON);
        assert_relative_eq!(res[1].y, 0.476012004174741, epsilon = f32::EPSILON);
        assert_relative_eq!(res[2].x, -0.6607183312158572, epsilon = f32::EPSILON);
        assert_relative_eq!(res[3].x, -0.07653668647301808);
        assert_relative_eq!(res[5].x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(res[5].y, 0.0, epsilon = f32::EPSILON);
    }
}

