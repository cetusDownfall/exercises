use piston_window::{Context, UpdateArgs, Size, Transformed};
use piston_window::ellipse;
use opengl_graphics::GlGraphics;
use entities::veloc::Veloc;
use entities::{Collide, Drawn, Positioned, Updated};
const PPM: f64 = 15.0;
const GRAVITY: f64 = 0.0;
const FRICTION: f64 = 0.1;
pub struct Ball {
    pub pos: Veloc,
    pub vel: Veloc,
    oob: bool, //out of bounds
    bounds: Size,
    diam: f64,
}
impl Ball {
    pub fn new(pos: Veloc, vel: Veloc, bounds: Size) -> Self {
        Ball {
            pos,
            vel,
            oob: false,
            bounds,
            diam: 50.0,
        }
    }
    pub fn oob(&self) -> bool {
        self.oob
    }
    pub fn diam(&self) -> f64 {
        self.diam
    }
    pub fn vel(&self) -> Veloc {
        self.vel
    }
    pub fn push(&mut self, force: Veloc) {
        self.vel += force;
    }
}
impl Updated for Ball {
    fn update(&mut self, args: UpdateArgs) {
        self.pos += self.vel;
        self.vel.y += PPM * GRAVITY * args.dt;
        if self.x() > self.bounds.width as f64 || self.x() < 0.0 {
            self.vel.x *= -(1.0 - FRICTION);
            if self.x() < 0.0 {
                self.pos.x = 0.0;
            }
            if self.x() > self.bounds.width as f64 {
                self.pos.x = self.bounds.width as f64;
            }
        }
        if self.y() > self.bounds.height as f64 || self.y() < 0.0 {
            self.vel.y *= -(1.0 - FRICTION);
            if self.y() < 0.0 {
                self.pos.y = 0.0;
            }
            if self.y() > self.bounds.height as f64 {
                self.pos.y = self.bounds.height as f64;
            }
        }
    }
}
impl Drawn for Ball {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        ellipse([1.0; 4], [self.diam / -2.0, self.diam / -2.0, self.diam, self.diam], context.transform.trans(self.pos.x, self.pos.y), graphics)
    }
}
impl Positioned for Ball {
    fn pos(&self) -> Veloc {
        self.pos
    }
}
impl Collide for Ball {
    type Target = Ball;
    fn radius(&self) -> f64 {
        self.diam() / 2.5
    }
}
