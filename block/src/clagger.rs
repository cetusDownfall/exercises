use std::env;
use std::str::FromStr;
pub struct Reader {
    lines: Vec<String>,
    index: usize,
}
pub enum LineType {
    Command,
    Other(Vec<String>),
}
impl Reader {
    pub fn new(lines: LineType) -> Reader {
        match lines {
            LineType::Command => {
                Reader {
                    lines: env::args().skip(1).collect::<Vec<String>>(),
                    index: 0,
                }
            },
            LineType::Other(s) => {
                Reader {
                    lines: s,
                    index: 0,
                }
            }
        }
    }
    pub fn assign_or<T: FromStr>(&mut self, o: T) -> T {
        if let Ok(n) = self.lines[self.index].parse::<T>() {
            self.index += 1;
            n
        } else {
            o
        }
    }
}
