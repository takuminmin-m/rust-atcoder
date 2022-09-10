use std::thread::current;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut b: [(i32, i32); n],
    }
    b.sort_by(|a, b| a.0.partial_cmp(&(b.0)).unwrap());

    let mut last_pos = b.last().unwrap().0;
    let mut current_time = std::cmp::max(last_pos, b.last().unwrap().1);
    for i in 0..n {
        current_time += last_pos - b[n-i-1].0;
        current_time = std::cmp::max(current_time, b[n-i-1].1);
        last_pos = b[n-i-1].0;
    }
    current_time += last_pos;

    println!("{}", current_time);
}
