use crate::utils::color::Color;
use crate::world::viewplane::ViewPlane;
use std::sync::Arc;
use crate::geometry::Geometry;
use std::cell::Cell;
use crate::output::OutputManager;

pub struct World
{
    m_backgroundcolor: Color,
    pub m_viewplaneptr: Box<ViewPlane>,
    pub m_objects: Cell<Vec<Arc<dyn Geometry>>>,
}

impl World
{
    pub fn new(viewplane: Box<ViewPlane>) -> World
    {
        World
        {
            m_backgroundcolor: Color::new(0.0, 0.0, 0.0),
            m_viewplaneptr: viewplane,
            m_objects: Cell::new(Vec::new()),
        }
    }

    pub fn setBackgroundColor(&mut self, newColor: Color)
    {
        self.m_backgroundcolor = newColor;
    }

    pub fn build(&mut self)
    {

    }

    pub fn addObject(&mut self, object: Arc<dyn Geometry>)
    {

    }

    pub fn removeObject(&mut self, index: i32)
    {

    }

    pub fn renderScene()
    {

    }

    pub fn writePixel(rownum: i32, colnum:i32, color: Color, omanager: Arc<dyn OutputManager>)
    {

    }
}

#[cfg(test)]
mod WorldTest
{
    use super::*;
    use crate::geometry::{sphere::Sphere};
    use cgmath::Vector3;

    #[test]
    fn checkAddObject()
    {
        /*
        let mut vpptr = Box::new(ViewPlane::new());
        let mut world = World::new(vpptr);
        let obj1 = Sphere::new(0.3, Vector3::new(0.8, 0.7, 0.6));
        let obj2 = Sphere::new(1.0, Vector3::new(2.0, 10.0, 8.0));
        let mut vecobject = Vec::new();
        vecobject.push(Arc::new(obj1));
        vecobject.push(Arc::new(obj2));
        world.addObject(Arc::new(obj1));
        world.addObject(Arc::new(obj2));
        assert_eq!(&(world.m_objects[0]), &(vecobject[0]));
        assert_eq!(&(world.m_objects[1]), &(vecobject[1]));
         */

    }

    #[test]
    fn checkRemoveObject()
    {

    }
}