extern crate piston_window;
use piston_window::{Context, Graphics, ImageSize, Transformed};
use piston_window::math::Vec2d;
use fauxsics::{Dimensioned, Positioned};
const TILE_SIZE: f64 = 16.0;
const WHITE: [f32;4] = [1.0; 4];
const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32;4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32;4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32;4] = [0.0, 0.0, 1.0, 1.0];
pub struct Tile(pub TileType, pub usize, pub usize); //x then y
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Blank,
    Basic,
    Red,
    Blue,
    Green,
}
impl Tile {
    pub fn new(t: TileType, x: usize, y: usize) -> Tile { Tile(t, x, y) }
}
pub trait Colored { fn color(&self) -> [f32;4]; } 
impl Colored for TileType { 
    fn color(&self) -> [f32;4] { 
        match *self {
            TileType::Blank => BLACK,
            TileType::Basic => WHITE,
            TileType::Red => RED,
            TileType::Green => GREEN,
            TileType::Blue => BLUE,
        }
    }
}
impl From<char> for TileType {
    fn from(l: char) -> TileType {
        match l {
            'r' => TileType::Red,
            'g' => TileType::Green,
            'b' => TileType::Blue,
            '#' => TileType::Basic,
            ' ' => TileType::Blank,
             _  => TileType::Blank,
        }
    }
}
impl From<(u8, u8, u8)> for TileType {
    fn from(c: (u8, u8, u8)) -> TileType {
        match c {
            (255, 255, 255) => TileType::Basic,
            (0, 0, 0) => TileType::Blank,
            _ => TileType::Blank,
        }
    }
}
