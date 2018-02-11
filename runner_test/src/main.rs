extern crate piston_window;
//extern crate fnv;
use piston_window::*;
use piston_window::Button::Keyboard;
mod fauxsics;
mod map;
mod camera;
use map::tile::*;
use map::Map;
use piston_window::math::Vec2d;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{min, max};
#[derive(Copy, Clone, PartialEq)]
pub enum Direction { Up, Down, Left, Right, Stop }
fn main() {
    const ALIVE: TileType = TileType::Green;
    const DEAD: TileType = TileType::Blank;
    let mut curr_pos: Vec2d = [47., 31.];
    let mut curr_acc: Vec2d = [0.0, 0.0];
    let mut curr_vel: Vec2d = [0.0, 0.0];
    let mut cam_vel: Vec2d = [0., 0.];
    let mut cam_pos: Vec2d = curr_pos;
    let mut window: PistonWindow =
        WindowSettings::new("GoL", [768, 512]).srgb(true).build().unwrap();
    window.set_max_fps(60);
    window.set_ups(120);
    let mut grid = Map::from_boxed_slice(vec![' ';256 * 256].into_boxed_slice(), 256, 256);
    grid.set_ratio(8.);
    let grid = Rc::new(RefCell::new(grid));
    let mut camera = camera::Camera::new(Rc::clone(&grid), curr_pos, [512., 768.]);
    while let Some(event) = window.next() {
        grid.borrow_mut().put_rec('g', curr_pos[0] as usize, curr_pos[1] as usize, 2, 2);
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics| camera.draw(context, graphics));
        }
        if let Some(args) = event.update_args() {
            let dt = args.dt;
            curr_vel[0] += curr_acc[0] * dt;
            curr_vel[1] += curr_acc[1] * dt;
            curr_pos[0] += curr_vel[0] * dt;
            curr_pos[1] += curr_vel[1] * dt;
            curr_pos[0] =
                if curr_pos[0] > 256. {
                    256.
                } else if curr_pos[0] < 0. {
                    0.
                } else {
                    curr_pos[0]
                };
            curr_pos[1] =
                if curr_pos[1] > 256. {
                    256.
                } else if curr_pos[1] < 0. {
                    0.
                } else {
                    curr_pos[1]
                };
            cam_pos[0] += cam_vel[0] * dt;
            cam_pos[1] += cam_vel[1] * dt;
            if cam_pos[0] < 0. { cam_pos[0] = 0.; }
            if cam_pos[1] < 0. { cam_pos[1] = 0.; }
            if cam_pos[0] > 256. { cam_pos[0] = 256.; }
            if cam_pos[1] > 256. { cam_pos[1] = 256.; }
            camera.set_focus([cam_pos[0], cam_pos[1]]);
            curr_vel[0] *= 0.6 * dt;
            if curr_vel[0].abs() <= 0.000005 {
                curr_vel[0] = 0.;
            }
            curr_vel[1] *= 0.6 * dt;
            if curr_vel[1].abs() <= 0.000005 {
                curr_vel [1] = 0.;
            }
            let mut d_tiles = Vec::new();
            for n in 0..grid.borrow().height() {
                for m in 0..grid.borrow().width() {
                   let mut neighbor_count = 0;
                   for a in 0..3usize {
                       let xo = a-1;
                       for b in 0..3usize {
                           let yo = b-1;
                           if xo == 0 && yo == 0 {
                               continue;
                           } else if grid.borrow().get_type(m-yo, n-xo) == ALIVE {
                               neighbor_count += 1;
                           }
                       }
                   }
                   match neighbor_count {
                      2  => {},
                      3 => d_tiles.push(Tile::new(ALIVE,m,n)),
                       _ => d_tiles.push(Tile::new(DEAD,m,n))
                   }
                }
            }
            for til in d_tiles {
                grid.borrow_mut().set(til);
            }
        }
        if let Some(args) = event.press_args() {
            if let Keyboard(key) = args {
                match key {
                    Key::W => curr_acc[1] = -1.,
                    Key::A => curr_acc[0] = -1.,
                    Key::S => curr_acc[1] = 1.,
                    Key::D => curr_acc[0] = 1.,
                    Key::Up => cam_vel[0] = -5.,
                    Key::Down => cam_vel[0] = 5.,
                    Key::Right => cam_vel[1] = 5.,
                    Key::Left => cam_vel[1] = -5.,
                    Key::Space => {cam_pos[0] = 128.; cam_pos[1] = 128.},
                    _ => {}
                }
            }
        }
        if let Some(args) = event.release_args() {
            if let Keyboard(key) = args {
                match key {
                    Key::W => if curr_acc[1] == -1. { curr_acc[1] = 0.},
                    Key::A => if curr_acc[0] == -1. { curr_acc[0] = 0. },
                    Key::S => if curr_acc[1] == 1. { curr_acc[1] = 0. },
                    Key::D => if curr_acc[0] == 1. { curr_acc[0] = 0. },
                    Key::Up | Key::Down => cam_vel[0] = 0.,
                    Key::Right | Key::Left => cam_vel[1] = 0.,
                    _ => {}
                }
            }
        }
    }
}
