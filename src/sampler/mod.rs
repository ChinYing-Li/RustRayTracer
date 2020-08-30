pub mod jittered;

use cgmath::{Vector2, Vector3};
use std::{f32, f32::consts::PI};
use approx::RelativeEq;
use cgmath::num_traits::Inv;

type Point2<T> = Vector2<T>;

pub struct SamplerCore
{
    pub m_sample_per_pattern: usize,
    pub m_num_pattern: usize,
    pub m_samples: Vec<Vector2<f32>>,
    pub m_samples_on_disk: Option<Vec<Vector2<f32>>>,
    pub m_samples_on_hemisphere: Option<Vec<Vector3<f32>>>,
}

impl SamplerCore
{
    fn new(sample_per_pattern: usize, num_pattern: usize) -> SamplerCore
    {
        SamplerCore
        {
            m_sample_per_pattern: sample_per_pattern,
            m_num_pattern: num_pattern,
            m_samples : Vec::with_capacity((sample_per_pattern as usize * num_pattern) as usize),
            m_samples_on_disk: None,
            m_samples_on_hemisphere: None,
        }
    }
}

pub trait Sampler
{
    fn generate_sample_pattern(&mut self);
    fn shuffle_indices(&mut self);
    fn setup_shuffle_indices(&mut self);
}

pub fn map_sample_to_disk(samplecore: &mut SamplerCore)
{
    if samplecore.m_samples_on_disk != None
    { return; }
    let mut radius = 0.0;
    let mut phi = 0.0;
    let mut samp = Vector2::new(0.0, 0.0);

    let n = samplecore.m_num_pattern * samplecore.m_sample_per_pattern;
    let mut diskpattern = Vec::with_capacity(n);
    let identityvec = Vector2::new(1.0, 1.0);

    for i in 0..n
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
            if samp.x < samp.y
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
        diskpattern.push(Vector2::new(radius*phi.cos(), radius*phi.sin()));
    }
    samplecore.m_samples_on_disk = Some(diskpattern);
}

fn mapSampleToHemisphere(core: &mut SamplerCore, e: f32)
{
    let n = core.m_num_pattern * core.m_sample_per_pattern;
    let mut hemisphere_pattern = Vec::with_capacity(n);
    for s in core.m_samples.iter()
    {
        let cos_phi = (2.0 * PI * s.x).cos();
        let sin_phi = (2.0 * PI * s.x).sin();
        let cos_theta = (1.0 - s.y).powf((e + 1.0).inv());
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();
        let pu = sin_theta * cos_phi;
        let pv = sin_theta * sin_phi;
        let pw = cos_theta;
        hemisphere_pattern.push(Vector3::new(pu, pv, pw));
    }
    core.m_samples_on_hemisphere = Some(hemisphere_pattern);
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
        core.m_samples.push(Vector2::new(0.8, 0.7));
        core.m_samples.push(Vector2::new(0.5, 0.6));
        core.m_samples.push(Vector2::new(0.7, 0.3));
        core.m_samples.push(Vector2::new(0.1, 0.2));
        core.m_samples.push(Vector2::new(0.5, 0.0));
        core.m_samples.push(Vector2::new(0.0, 0.0));
        // place holder
        core.m_samples.push(Vector2::new(0.0, 0.0));
        core.m_samples.push(Vector2::new(0.0, 0.0));

        map_sample_to_disk(&mut core);

        assert_relative_eq!(core.m_samples_on_disk.clone().unwrap()[0].y, 0.2999999999999, epsilon = f32::EPSILON);
        assert_relative_eq!(core.m_samples_on_disk.clone().unwrap()[1].y, 0.0000000000, epsilon = f32::EPSILON);
        /*
        assert_relative_eq!(res[2].x, -0.6607183312158572, epsilon = f32::EPSILON);

        assert_relative_eq!(res[3].x, -0.07653668647301808);
        assert_relative_eq!(res[5].x, 0.0, epsilon = f32::EPSILON);
        assert_relative_eq!(res[5].y, 0.0, epsilon = f32::EPSILON);*/
    }
}

