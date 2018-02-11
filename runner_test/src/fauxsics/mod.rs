extern crate piston_window;
use piston_window::math::Vec2d;
pub trait Positioned {
    fn pos(&self) -> Vec2d;
    fn x(&self) -> f64 { self.pos()[0] }
    fn y(&self) -> f64 { self.pos()[1] }
}
pub trait Dimensioned {
    fn dim(&self) -> Vec2d;
    fn width(&self) -> f64 { self.dim()[0] }
    fn height(&self) -> f64 { self.dim()[1] }
    fn area(&self) -> f64 { self.width() * self.height() }
}
