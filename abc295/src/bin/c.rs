use std::collections::HashMap;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let mut h = HashMap::new();
    for _ in 0..n {
        input! {
            a: u64,
        }

        *h.entry(a).or_insert_with(|| 0) += 1;
    }

    let mut res = 0;
    for (_col, a) in h.iter() {
        res += a / 2;
    }

    println!("{}", res);
}
