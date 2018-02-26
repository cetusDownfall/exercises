use piston_window::keyboard::*;
use piston_window::*;
use std::cell::RefCell;
use wire_grid::*;
use wire_cells::*;
use wire_anim::*;
use std::rc::Rc;
#[derive(Copy,Clone)]
pub struct Cursor {
    pos: (i64, i64),
    bounds: (i64, i64),
}
impl Cursor {
    pub fn new(pos: (i64, i64), bounds: (i64, i64)) -> Cursor {
        Cursor { pos, bounds }
    }
    pub fn pos(&self) -> (i64, i64) {
        self.pos
    }
    pub fn up(&mut self) { self.pos.0 = ((self.pos.0 % self.bounds.0) + self.bounds.0 - 1) % self.bounds.0 }
    pub fn down(&mut self) { self.pos.0 = ((self.pos.0 % self.bounds.0) + self.bounds.0 + 1) % self.bounds.0 }
    pub fn left(&mut self) { self.pos.1 = ((self.pos.1 % self.bounds.1) + self.bounds.1 - 1) % self.bounds.1 }
    pub fn right(&mut self) { self.pos.1 = ((self.pos.1 % self.bounds.1) + self.bounds.1 + 1) % self.bounds.1 }
    pub fn head(&self) -> Option<WireCell> { Some(WireCell::new(WState::EHead, self.pos.1, self.pos.0)) }
    pub fn tail(&self) -> Option<WireCell> { Some(WireCell::new(WState::ETail, self.pos.1, self.pos.0)) }
    pub fn wire(&self) -> Option<WireCell> { Some(WireCell::new(WState::Conductor, self.pos.1, self.pos.0)) }
    pub fn dead(&self) -> Option<WireCell> { None }
}
pub trait Editor<C,G> 
where C: WCell, G: WGrid<C>
{
    fn grid(&self) -> Rc<G>;
    fn grid_mut(&mut self) -> Option<&mut G>;
    fn display(&self, c: Context, g: &mut G2d);
    fn cursor(&self) -> Rc<RefCell<Cursor>>;
    fn place(&mut self, cell: Option<C>, pos: (i64, i64)) {
        if let Some(g) = self.grid_mut() {
            g.put(cell, (pos.0.into(), pos.1.into()));
        }
    }
    fn cursor_put(&mut self, cell: Option<C>) {
        let pos = self.cursor();
        self.place(cell, pos.borrow().pos());
    }
}
pub struct SimpleEdit<G: WGrid<WireCell>> {
    conts: Rc<G>,
    cursor: Rc<RefCell<Cursor>>,
}
impl<G: WGrid<WireCell>> Editor<WireCell,G> for SimpleEdit<G> {
    fn grid(&self) -> Rc<G> {
        Rc::clone(&self.conts)
    }
    fn grid_mut(&mut self) -> Option<&mut G> {
        Rc::get_mut(&mut self.conts)
    }
    fn display(&self, c: Context, g: &mut G2d) {
        clear([0., 0., 0., 0.], g);
        for cl in self.grid().get_cells() {
            let (x, y) = cl.pos();
            let col =
                match cl.get_state() {
                    WState::Conductor => [1., 1., 1., 1.],
                    WState::EHead => [1., 0., 0., 1.],
                    WState::ETail => [0.5, 0.5, 0.5, 1.0],
                };
            rectangle(col, [1., 1., 5., 5.],
                      c.transform.trans(x as f64 * 7., y as f64 * 7.), g);
        }
        let cur = self.cursor();
        rectangle([0., 1., 1., 0.6], 
                                 [0., 0., 7., 7.], 
                                 c.transform.trans(cur.borrow().pos().1 as f64 * 7.,
        cur.borrow().pos().0 as f64 * 7.), g);
    }
    fn cursor(&self) -> Rc<RefCell<Cursor>> {
        Rc::clone(&self.cursor)
    }
}
impl<G: WGrid<WireCell>> From<G> for SimpleEdit<G> {
    fn from(grid: G) -> SimpleEdit<G> {
        let conts = Rc::new(grid);
        SimpleEdit {conts: Rc::clone(&conts), cursor: Rc::new(RefCell::new(Cursor::new((0, 0), 
                        (conts.bounds().0.into(), conts.bounds().1.into()))))}
    }
}

impl SimpleEdit<WireGrid> {
    pub fn new(size: (i64, i64)) -> SimpleEdit<WireGrid> {
        WireGrid::new(size).into()
    }
    pub fn start(&mut self) {
        let mut window: PistonWindow =
            WindowSettings::new("Simple Edit", 
                                [self.conts.bounds().0 as u32* 7,
                                 self.conts.bounds().1 as u32* 7])
            .build().unwrap();
        while let Some(e) = window.next() {
            if let Some(args) = e.render_args() {
                window.draw_2d(&e, |c,g| {
                    self.display(c,g);
                });
            }
            let cur = self.cursor();
            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::W => cur.borrow_mut().up(),
                    Key::A => cur.borrow_mut().left(),
                    Key::S => cur.borrow_mut().down(),
                    Key::D => cur.borrow_mut().right(),
                    Key::J => self.cursor_put(cur.borrow().head()),
                    Key::K => self.cursor_put(cur.borrow().tail()),
                    Key::L => self.cursor_put(cur.borrow().dead()),
                    Key::Space => self.cursor_put(cur.borrow().wire()),
                    Key::Right => self.grid().next_grid_state(),
                    Key::Return => {
                        let grid_t = self.grid();
                        SimpleAnim::easy_anim(grid_t);
                        },
                    _ => {}
                }
            }
        }
    }
}
