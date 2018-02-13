use opengl_graphics::GlGraphics;
use piston_window::{Context, UpdateArgs};
use entities::veloc::Veloc;
pub mod ball;
pub mod veloc;
pub trait Drawn {
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}
pub trait Updated {
    fn update(&mut self, args: UpdateArgs);
}
pub trait Positioned {
    fn dist<T: Positioned>(&self, other: T) -> f64 {
        let crow = self.pos() - other.pos();
        (crow.x.powi(2) + crow.y.powi(2)).sqrt()
    }
    fn x(&self) -> f64 {
        self.pos().x
    }

    fn y(&self) -> f64 {
        self.pos().y
    }

    fn pos(&self) -> veloc::Veloc;
}
pub trait Collide: Positioned {
    type Target: Positioned + Collide;
    fn radius(&self) -> f64;
    fn collide(&self, other: &Self::Target) -> bool {
        self.pos().dist(other.pos()) < self.radius() + other.radius()
    }
}
