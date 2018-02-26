use piston_window::*;
use wire_edit::*;
use wire_grid::*;
use wire_cells::*;
use std::rc::Rc;
use std::time::{Instant, Duration};
pub trait Anim<C: WCell, G: WGrid<C>>
where Self: From<Rc<G>>
{
    fn grid(&self) -> Rc<G>;
    fn initialise(&mut self);
    fn show(&mut self);
    fn finish(&self) -> bool;
    fn play(&mut self) {
        self.initialise();
        while !self.finish() {
            self.show();
            self.grid().next_grid_state();
        }
    }
    fn easy_anim(grid: Rc<G>) {
        Self::from(grid).play();
    }
}
pub struct SimpleAnim<G: WGrid<WireCell>>(Rc<G>, PistonWindow, bool);
impl From<Rc<WireGrid>> for SimpleAnim<WireGrid> {
    fn from(grid: Rc<WireGrid>) -> SimpleAnim<WireGrid> {
        SimpleAnim(Rc::clone(&grid),
            WindowSettings::new("Simple Anim", 
                                [grid.bounds().0 as u32* 8,
                                 grid.bounds().1 as u32* 8])
            .build().unwrap(),false)
    }
}
impl Anim<WireCell,WireGrid> for SimpleAnim<WireGrid> {
    fn grid(&self) -> Rc<WireGrid> {
        Rc::clone(&self.0)
    }
    fn initialise(&mut self) {
        self.0.refresh_all();
        self.2 = false;
    }
    fn show(&mut self) {
        while let Some(e) = self.1.next() {
            if let Some(args) = e.render_args() {
                let cls = self.grid();
                self.1.draw_2d(&e, |c,g| {
                    clear([0., 0., 0., 1.], g);
                    for cl in cls.get_cells() {
                        let (x, y) = cl.pos();
                        let col =
                            match cl.get_state() {
                                WState::Conductor => [0., 0., 0., 1.],
                                WState::EHead => [1., 0., 0., 1.],
                                WState::ETail => [0.5, 0.5, 0.5, 1.0],
                            };
                        rectangle(col, [0., 0., 8., 8.],
                                  c.transform.trans(x as f64 * 8., y as f64 * 8.), g);
                    }
                });
            }
            if let Some(args) = e.update_args() {
                self.grid().next_grid_state();
            }
            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::E => self.2 = true,
                    _ => {}
                }
            }
            if self.2 {
                break;
            }
        }
    }
    fn finish(&self) -> bool {
        self.2
    }
}

pub struct TextAnim<G: WGrid<WireCell>>(Rc<G>, u64, Duration); 
impl<G: WGrid<WireCell>> From<Rc<G>> for TextAnim<G> {
    fn from(grid: Rc<G>) -> Self {
        TextAnim(grid,0,Duration::from_millis(0))
    }
}
impl Anim<WireCell,WireGrid> for TextAnim<WireGrid> {
    fn grid(&self) -> Rc<WireGrid> {
        Rc::clone(&self.0)
    }
    fn initialise(&mut self) {
        self.0.refresh_all();
        self.2 = Duration::from_millis(self.1);
    }
    fn show(&mut self) {
        let chrep = 
            |st: WState| -> char {
                match st {
                    WState::Conductor => ' ',
                    WState::EHead => 'O',
                    WState::ETail => 'o',
                }
            };
        println!();
        let mut r = 0;
        let mut c = 0;
        let cells = self.0.get_cells();
        while r < self.0.bounds().0 {
            println!();
            let row = cells.iter().filter(|cl| cl.pos().0 == r)
                .map(|cl| Rc::clone(&cl))
                .collect::<Vec<Rc<WireCell>>>();
            c = 0;
            while c < self.0.bounds().1 {
                print!("{}",
                if let Some(cl) = row.iter().find(|ct| ct.pos().1 == c) 
                { chrep(cl.get_state()) } 
                else 
                { ' ' });
                c += 1;
            }
            r += 1;
        }
        println!();
    }
    fn finish(&self) -> bool {
        false
    }
}
