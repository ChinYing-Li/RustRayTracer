pub mod matte;
pub mod phong;

use crate::utils::shaderec::ShadeRec;
use crate::utils::color::Colorf;

pub trait Material
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf;
  //  fn areaLightShade<'a>(&self, sr: &'a mut ShadeRec);
   // fn pathShade<'a>(&self, sr: &'a mut ShadeRec);
}
