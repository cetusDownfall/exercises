extern crate piston_window;
use piston_window::{ Context, Graphics, ImageSize, Transformed, rectangle };
use std::cell::RefCell;
use std::rc::Rc;
use piston_window::math::{Vec2d};
use fauxsics::{ Dimensioned, Positioned };
pub mod tile;
use self::tile::{Tile, Colored, TileType};
fn get_index(width: usize, x: usize, y: usize) -> usize { (width * y) + x }
pub struct Map {
    conts: Box<[TileType]>,
    height: usize, 
    width: usize,
    ratio: f64,
    f_height: f64,
    f_width: f64
}
impl Map {
    pub fn from_boxed_slice<T: Into<TileType>>(s: Box<[T]>, height: usize, width: usize) -> Map {
        let out: Vec<T> = s.into();
        let s: Box<[TileType]> =
           out.into_iter() 
              .map(|c| c.into())
              .collect::<Vec<TileType>>()
              .into_boxed_slice();
        //Default float dims set to converted tile count.
        Map { 
            conts: s,
            height, width, 
            ratio: 1., 
            f_height: height as f64, f_width: width as f64 
        }
    }
    pub fn ratio(&self) -> f64 { self.ratio }
    pub fn height(&self) -> usize { self.height }
    pub fn width(&self) -> usize { self.width }
    pub fn f_height(&self) -> f64 { self.f_height}
    pub fn f_width(&self) -> f64 { self.f_width }
    pub fn set_ratio(&mut self, ratio: f64) {
        self.f_height = self.height() as f64 * ratio;
        self.f_width = self.width() as f64 * ratio;
        self.ratio = ratio;
    }
    pub fn get_type(&self, x: usize, y: usize) -> TileType { 
        self.conts[self.safe_ind(x, y)] 
    }

    //Expensive. But helpful in some cases I bet.
    pub fn get_tile(&self, x: usize, y: usize) -> Tile { 
        Tile::new(self.conts[self.safe_ind(x, y)], x, y) } 

    pub fn set(&mut self, t: Tile) {
        self.conts[self.safe_ind(t.1, t.2)] = t.0;
    }

    pub fn set_tile(&mut self, cr: TileType, x: usize, y: usize) {
        self.conts[self.safe_ind(x, y)] = cr;
    }
    pub fn put_rec<T: Into<TileType>>(&mut self,
          cr: T,
          x: usize,
          y: usize,
          w: usize,
          h: usize)
    {
        let cr: TileType = cr.into();
        for r in y..(y + h) {
            for c in x..(x + w) {
                self.set_tile(cr, c, r)
            }
        }
    }
    fn safe_ind(&self, x: usize, y: usize) -> usize { get_index(self.width, x, y) % (self.height * self.width) }

    pub fn draw<G, T>(&self, d: f64, context: Context, graphics: &mut G)
    where G: Graphics<Texture = T>, T: ImageSize {
        for r in 0..self.height {
            for c in 0..self.width {
                rectangle(self.get_type(c, r).color(),
                          [0.0, 0.0, d, d],
                          context.transform.trans(c as f64 * d, r as f64 * d),
                          graphics);
            }
        }
    }
    pub fn draw_section<G, T>(&self, pos: Vec2d, dim: Vec2d, context: Context, graphics: &mut G) 
    where G: Graphics<Texture = T>, T: ImageSize {
        let y_off = pos[0] / self.ratio;
        let x_off = pos[1] / self.ratio;
        let a_dim_y = dim[0] / self.ratio;
        let a_dim_x = dim[1] / self.ratio;
        for r in (y_off as usize)..((y_off + a_dim_y) as usize) {
            for c in (x_off as usize)..((x_off + a_dim_x) as usize) {
                rectangle(self.get_type(c, r).color(),
                          [0.0, 0.0, self.ratio, self.ratio],
                          context.transform.trans((c as f64 * self.ratio) - pos[1],
                                                  (r as f64 * self.ratio) - pos[0]),
                          graphics);
            }
        }
    }
}
