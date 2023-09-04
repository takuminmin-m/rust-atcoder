use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let mut vec = Vec::new();

    for i in 0..n {
        input! {
            a: u128,
            b: u128,
        }

        vec.push((i+1, b, a+b));
    }

    vec.sort_by(|a, b| (a.1*b.2).partial_cmp(&(b.1*a.2)).unwrap());
    for i in 0..n-1 {
        print!("{} ", vec[i].0);
    }
    println!("{}", vec[n-1].0);
}
