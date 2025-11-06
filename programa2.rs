fn transpose<T: Clone>(m: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if m.is_empty() { return vec![]; }
    let rows = m.len();
    let cols = m[0].len();
    (0..cols)
        .map(|c| (0..rows).map(|r| m[r][c].clone()).collect())
        .collect()
}

fn main() {
    let x = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    let xt = transpose(x);
    for row in xt {
        println!("{row:?}");
    }
}
