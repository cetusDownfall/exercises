fn main() {
    let vect = vec![
        vec![0, 1, 2, 3],
        vec![4, 5, 6, 7],
        vec![8, 9, 10, 11],
        vec![12, 13, 14, 15]];
    println!("{:?}", vect.into_iter().flat_map(|it| it.into_iter()).collect::<Vec<i32>>());
}
