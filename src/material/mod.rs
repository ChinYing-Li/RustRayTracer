pub mod matte;
pub mod phong;

use crate::utils::shaderec::ShadeRec;

pub trait Material
{
    fn shade<'a>(sr: &'a mut ShadeRec);
    fn areaLightShade<'a>(sr: &'a mut ShadeRec);
    fn pathShade<'a>(sr: &'a mut ShadeRec);
}
