//Still nonfunctional, nor is it near what I want to practice yet.
pub struct Grid(Vec<Vec<i32>>);
impl Grid {
    pub fn new(mut source: Count, width: usize, height: usize) -> Grid {
        let mut contents: Vec<Vec<i32>> = Vec::with_capacity(width * height);
        for _ in 0..height {
            let mut row: Vec<i32> = Vec::with_capacity(width);
            for _ in 0..width {
                let a: i32 = match source.next() { Some(n) => n, None => 0, };
                row.push(a);
            }
            contents.push(row);
        }
        Grid(contents)
    }
}
impl IntoIterator for Grid {
    type Item = i32;
    type IntoIter = std::vec::Drain<'static, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().flat_map(|it| it.drain(..)).collect::<Vec<i32>>().drain(..)
        /*let mut a = self.0.iter();
        let mut out = a.next().unwrap().iter();
        while let Some(row) = a.next() {
            out = out.chain(row);
        }
        out */
        //a.fold(out, |acc, next| acc.chain(next.iter_mut())).collect::<Vec<i32>>().iter()
    }
}
pub struct Count(i32);
impl Count {
    pub fn new() -> Count {
        Count(0)
    }
}
impl Iterator for Count {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        self.0 += 1;
        Some(self.0)
    }
}
fn main() {
    let mut tick = Count::new();
    let mut test = Grid::new(tick, 5, 5);
    for item in test {
        println!("{}", item);
    }
}
