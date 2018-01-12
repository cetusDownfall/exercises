extern crate piston_window;
extern crate block;
use piston_window::*;
use block::margolus_gas::Gas;
use block::clagger::{LineType, Reader};

fn main() {
    let mut args = Reader::new(LineType::Command);
    let grid_size = args.assign_or(128usize);
    let hole_size = args.assign_or(100usize);
    let block_size = args.assign_or(128usize);
    let density = args.assign_or(0.3f32);
    let framed = args.assign_or(true);
    let frames = args.assign_or(500usize);
    let ratio = args.assign_or(8.0f64);
    let shape: [f64; 4] = [0.0, 0.0, ratio, ratio];
    let mut grid: Gas = Gas::new(grid_size, density, true);
    grid.hole(hole_size);
    grid.block(block_size);
    let mut window: PistonWindow =
        WindowSettings::new("Gas", [grid_size as u32 * (0.5 + ratio/2.0) as u32, grid_size as u32 * (0.5 + ratio/2.0) as u32])
            .exit_on_esc(true).build().unwrap();
    let trans = |c: piston_window::Context, i, j| c.transform.trans(i, j);
    if framed {
        let mut frame_vec: Vec<Vec<(f64, f64, u8)>> = grid.generate_frames(frames);
        frame_vec.reverse();
        println!("Generated {} frames", frames);
        while let Some(event) = window.next() {
            if let Some(points) = frame_vec.pop() {
                let points = 
                    points.into_iter()
                          .filter(|&(_, _, n)| n > 0)
                          .map(|(i, j, n)| (i * ratio, j * ratio, [1.0, 1.0, 1.0, n as f32 * 0.25]))
                          .collect::<Vec<(f64, f64, [f32; 4])>>();
                window.draw_2d(&event, |c, g| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    points.into_iter().for_each(|(i, j, n)|
                          rectangle(n,
                                    shape,
                                    trans(c, i, j), g));
                });
            } else {
                println!("No more frames");
                break;
            }
        }
    } else {
        while let Some(event) = window.next() {
            let points = grid.pass_state();
            window.draw_2d(&event, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                points.into_iter()
                      .filter(|&(_, _, n)| n > 0).for_each(|(i, j, n)| {
                      rectangle([1.0, 1.0, 1.0, n as f32 * 0.25],
                                shape,
                                trans(c, i * ratio, j * ratio), g)
                      });
            });
        }
    }
}
