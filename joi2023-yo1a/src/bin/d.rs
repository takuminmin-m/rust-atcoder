use std::iter::Iterator;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: [i32; 2*n-1],
    }

    let a_sum: i32 = a.iter().sum();
    let norm_sum = ((n+1)*n) as i32;
    println!("{}", norm_sum-a_sum);
}
