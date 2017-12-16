pub fn pivot(target: &mut [u8]) {
    if target.len() <= 1 { return; }
    let p = target[target.len() / 2];
    let mut side_l = 0usize;
    let mut side_r: usize = target.len()-1;
    while side_l <= side_r {
        while target[side_l] < p {
            side_l += 1;
        }
        while target[side_r] > p {
            side_r -= 1;
        }
        if side_l <= side_r {
            let x = target[side_l];
            let y = target[side_r];
            target[side_r] = x;
            target[side_l] = y;
            side_l += 1;
            side_r -= 1;
        }
    }
    pivot(&mut target[..side_r]);
    pivot(&mut target[side_l..]);
}
