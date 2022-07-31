use proconio::{ input, marker::Chars };
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = 0_usize;
    for i in 0..n {
        if a[i] == i+1 {
            count += 1;
        }
    }

    let mut res = 0;
    for i in 1..count {
        res += i;
    }

    for i in 0..(n-1) {
        if a[a[i]-1] == i+1 && a[a[i]-1] < a[i] {
            // println!("i{} j{}", i+1, a[i]);
            res += 1;
        }
    }

    println!("{}", res);
}
