mod world;

use std::sync::{Arc};
use std::{boxed::Box};
use crate::utils::color::Color;
use crate::geometry::{sphere::Sphere, Geometry};

use self::viewplane::ViewPlane;
use std::cell::Cell;

pub mod viewplane;
