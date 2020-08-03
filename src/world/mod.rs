use std::sync::{Arc};
use std::{boxed::Box};
use crate::utils::color::Colorf;
use crate::geometry::{sphere::Sphere, Geometry};

use self::viewplane::ViewPlane;

pub mod viewplane;
pub mod world;
