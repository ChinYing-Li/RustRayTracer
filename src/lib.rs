#![allow(dead_code)]

pub mod brdf;
pub mod render;
//pub mod gallery;
pub mod geometry;
pub mod light;
pub mod material;
pub mod math;
pub mod output;
pub mod ray;
pub mod sampler;
pub mod tracer;
pub mod utils;
pub mod world;

extern crate arrayvec;
extern crate enum_set as enum_set;
extern crate obj;
extern crate rand;
extern crate time;
extern crate scoped_threadpool;
extern crate image;
extern crate cgmath;
extern crate approx;
