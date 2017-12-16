fn main() {
    const NUM_DISCS: i32 = 32;
    const WIDTH: i32 = 1 + NUM_DISCS*2;
    //const TERM_SIZE: i32 = 57;
    fn disc(size: &i32) -> String {
        if *size <= 0 as i32 {
            String::from("|")
        } else {
            let mut out = String::from("-");
            let mut i: usize = 1;
            while i < *size as usize {
                out = out + "--";
                i += 1;
            }
            out
        }
    }
    let mut towers: Vec<Vec<i32>> = vec![Vec::new(), Vec::new(), Vec::new()];
    for i in (0..NUM_DISCS).rev() {
        towers[0].push(i+1);
    }
    fn space_string(size: &i32) -> String { let mut out = String::from(" "); while out.len() < *size as usize { out = out + " "; } out }
    fn char_string(size: &i32, ch: &str) -> String { let mut out = String::from(ch); while out.len() < *size as usize { out = out + &String::from(ch); } out }
    fn print_towers(tower_set_1: &Vec<Vec<i32>>) {
        let mut lines: Vec<String> = Vec::new();
        let heights: (usize, usize, usize) = (tower_set_1[0].len(), tower_set_1[1].len(), tower_set_1[2].len());
        let (x, y, z) = heights;
        let height = highest_element(&heights);
        lines.push(char_string(&((WIDTH + 1) * 9 + 1), &String::from("\u{25A0}")));
        for i in (0..*height).rev() {
            let tower_1: String = disc(&(if i < x { tower_set_1[0][i] } else { -1 }));
            let tower_2: String = disc(&(if i < y { tower_set_1[1][i] } else { -1 }));
            let tower_3: String = disc(&(if i < z { tower_set_1[2][i] } else { -1 }));
            let padding_1: String = space_string(&((WIDTH - tower_1.len() as i32) / 2));
            let padding_2: String = space_string(&((WIDTH - tower_2.len() as i32) / 2));
            let padding_3: String = space_string(&((WIDTH - tower_3.len() as i32) / 2));
            lines.push(String::from(format!("\u{250A}{}{}{}\u{250A}{}{}{}\u{250A}{}{}{}\u{250A}",
                               padding_1, tower_1, padding_1,
                               padding_2, tower_2, padding_2,
                               padding_3, tower_3, padding_3)));
        }
        //for _i in 0..(TERM_SIZE - (*height as i32 + 2)) {
         //   println!();
        //}
        for i in &lines {
            println!("{}", i);
        }
    }
    fn move_disc(from: usize, to: usize, tower_set: &mut Vec<Vec<i32>>) {
        if from < 3 && to < 3  && tower_set[from].len() > 0 {
            let f = tower_set[from].pop().unwrap();
            tower_set[to].push(f);
        }
    }

    fn highest_element(tup: &(usize, usize, usize)) -> &usize {
        let tups: Vec<&usize> = vec![&tup.0, &tup.1, &tup.2];
        let mut highest_so_far: &usize = tups[0];
        for i in tups {
            if i > highest_so_far { highest_so_far = i; }
        }
        highest_so_far
    }
    fn han<'a>(num: i32, from: &'a usize, via: &'a usize, to: &'a usize, tower_set: &mut Vec<Vec<i32>>) {
        if num == 0 { 
            print_towers(&tower_set);
        } else if num == 1 { 
            println!("Move disc from {} to {}.", from, to); 
            move_disc(*from, *to, tower_set); 
            han(num-1, from, to, via, tower_set);
        } else { 
            han(num-1, from, to, via, tower_set); 
            han(1, from, via, to, tower_set); 
            han(num-1, via, from, to, tower_set); 
        }
    }
    han(NUM_DISCS, &0, &1, &2, &mut towers);
    println!("Done");
}
