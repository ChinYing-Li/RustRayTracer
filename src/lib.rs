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

extern crate enum_set as enum_set;
extern crate rand;
extern crate obj;
extern crate arrayvec;
extern crate scoped_threadpool;
#[macro_use]
extern crate image;
extern crate cgmath;
extern crate approx;
