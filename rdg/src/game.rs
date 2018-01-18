use piston_window::{
    Event, clear, Button, Motion, Context, Key, MouseButton, MouseCursorEvent, PistonWindow, PressEvent, ReleaseEvent, RenderEvent, Size, UpdateArgs, UpdateEvent};
use entities::{Collide, Drawn, Updated, Positioned};
use entities::ball::Ball;
use entities::veloc::Veloc;
use opengl_graphics::GlGraphics;

pub struct Game {
    balls: Vec<Ball>,
    window_size: Size,
    mouse_loc: Veloc,
    mouse_on: Option<Veloc>,
}
impl Game {
    pub fn new(window_size: Size) -> Self {
        Game {
            balls: Vec::new(),
            window_size,
            mouse_loc: [0.0, 0.0].into(),
            mouse_on: None,
        }
    }
    pub fn run(&mut self,
               window: &mut PistonWindow,
               opengl: &mut GlGraphics,
               ) {
        while let Some(e) = window.next() {
            if let Some(args) = e.render_args() {
                opengl.draw(args.viewport(), |c, g| {
                    self.draw(c, g)
                });
            }
            if let Some(args) = e.update_args() {
                self.update(args);
            }
            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::Space => {self.balls.pop();},
                    _ => {}
                }
            }
            if let Some(Button::Keyboard(key)) = e.release_args() {
                match key {
                    Key::Up => self.balls[0].push([0.0, -5.0].into()),
                    Key::Down => self.balls[0].push([0.0, 5.0].into()),
                    Key::Right => self.balls[0].push([5.0, 0.0].into()),
                    Key::Left => self.balls[0].push([-5.0, 0.0].into()),
                    _ => {}
                }
            }
            if let Some(loc) = e.mouse_cursor_args() {
                self.mouse_loc = loc.into();
            }
            if let Some(Button::Mouse(button)) = e.press_args() {
                match button {
                    MouseButton::Left => self.mouse_on = Some(self.mouse_loc), 
                    _ => {}
                }
            }
            if let Some(Button::Mouse(button)) = e.release_args() {
                if let Some(first) = self.mouse_on {
                    match button {
                        MouseButton::Left => self.balls.push(Ball::new(first, (self.mouse_loc - first)/10.0, self.window_size)),
                        _ => {},
                    }
                }
            }
        }
    }
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);
        for ball in &self.balls {
            ball.draw(context, graphics);
        }
    }
}
impl Updated for Game {
    fn update(&mut self, args: UpdateArgs) {
        for ball in &mut self.balls {
            ball.update(args);
        }
        let mut bump: bool = false;
        if let (Some(ball), Some(ball_a)) = (self.balls.get(0), self.balls.get(1)) {
            bump = ball.collide(ball_a);
        }
        if bump {
            if let Some((ball, rest)) = self.balls.split_first_mut() {
                rest[0].vel *= -1.0;
                rest[0].vel += ball.vel;
                ball.vel -= rest[0].vel;
                rest[0].vel += ball.vel;
            }
        }
    }
}
