extern crate piston_window;
use map::Map;
use std::cell::RefCell;
use std::rc::Rc;
use piston_window::{Context, Graphics, ImageSize};
use piston_window::math::Vec2d;
use fauxsics::{Dimensioned, Positioned};
pub struct Camera {
    target: Rc<RefCell<Map>>,
    pos: Vec2d, //x y?
    dim: Vec2d
}

impl Positioned for Camera { fn pos(&self) -> Vec2d { self.pos } }
impl Dimensioned for Camera { fn dim(&self) -> Vec2d { self.dim } }
impl Camera {
    pub fn draw<G, T>(&self, context: Context, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize {
            self.target.borrow().draw_section(self.pos(), self.dim(), context, graphics); }
    pub fn set_focus(&mut self, foc: Vec2d) {
        let mut x = 
            ((foc[0] * self.target.borrow().ratio()) - self.dim[0]/2.)
             .min(self.target.borrow().f_width() - (self.dim[0] / 2.));
        let mut y = 
            ((foc[1] * self.target.borrow().ratio()) - self.dim[1]/2.)
             .min(self.target.borrow().f_height() - (self.dim[1] / 2.));
        x = x.max(self.dim[0] / 2.);
        y = y.max(self.dim[1] / 2.);
        self.pos = [x, y];
    }
    pub fn new(target: Rc<RefCell<Map>>, pos: Vec2d, dim: Vec2d) -> Camera {
        Camera { target, pos, dim }
    }
}
