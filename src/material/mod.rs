pub mod matte;
pub mod phong;

use crate::utils::shaderec::ShadeRec;
use crate::utils::color::Colorf;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::error::Error;


pub trait Material
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf;
  //  fn areaLightShade<'a>(&self, sr: &'a mut ShadeRec);
   // fn pathShade<'a>(&self, sr: &'a mut ShadeRec);
}

impl Debug for Material
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Material")
            .finish()
    }
}
