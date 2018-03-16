extern crate hexapwn;
use std::io;
use hexapwn::*;
fn main() {
    let mut board = Board::new();
    let mut is_over = false;
    let mut next_move: String;
    while !is_over {
        next_move = String::from("");
        println!("{}'s turn: ", match board.side() {
            Side::Black => "Black",
            Side::White => "White",
        });
        println!("{}", board.simple_board());
        if let Ok(_) = io::stdin().read_line(&mut next_move) {
            let cos = next_move.chars().take(2).map(|c| {
                match c {
                    'a'|'3' => 0,
                    'b'|'2' => 1,
                    'c'|'1' => 2,
                     _      => 3,
                }
            }).collect::<Vec<isize>>();
            if next_move.len() > 4 {
                match board.capture(cos[1], cos[0],
                    match next_move.chars().skip(2).next().unwrap() {
                        'l' => Direction::Left,
                        'r' => Direction::Right,
                         _ => Direction::Left, 
                    }) {
                    Ok(b) => {
                        is_over = b;
                        board.switch_side();
                    },
                    Err(em) => println!("{}", em),
                }
            } else {
                match board.advance(cos[1], cos[0]) {
                    Ok(b) => {
                        is_over = b;
                        board.switch_side();
                    },
                    Err(em) => println!("{}", em),
                }
            }
        }
    }
}

