use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut g = vec![Vec::<usize>::new(); n];
    for i in 0..(n-1) {
        input! {
            a: usize,
            b: usize,
        }
        g[a-1].push(b-1);
        g[b-1].push(a-1);
    }

    let mut seen = vec![false; n];
    println!("{:?}", s(&g, &mut seen, n, 0))
}
