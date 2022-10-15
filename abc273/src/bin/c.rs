use std::vec;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    a.sort();
    let mut b = vec![1; 1];
    let mut b_len = 0;
    for i in 1..n {
        if a[i-1] == a[i] {
            b[b_len] += 1;
        } else {
            b.push(1);
            b_len+=1;
        }
    }

    a.dedup();
    for i in 1..=n {
        if a.len() < i {
            println!("0");
        } else {
            println!("{}", b[b.len()-i]);
        }
    }
}
