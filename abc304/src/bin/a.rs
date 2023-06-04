use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let mut s = Vec::new();
    let mut a = Vec::new();
    for i in 0..n {
        input! {
            s1: String,
            a1: usize,
        }

        s.push(s1);
        a.push(a1)
    }

    let mut min = 9999999999;
    let mut min_i = 0;
    for (i, val) in a.iter().enumerate() {
        if min > *val {
            min = *val;
            min_i = i;
        }
    }

    for i in 0..n {
        println!("{}", s[(i+min_i)%n]);
    }
}
