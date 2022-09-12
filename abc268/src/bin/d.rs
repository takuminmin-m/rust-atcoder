use std::vec;

use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n],
        t: [Chars; m],
    }

    let mut len = 0;
    for i in 0..s.len() {
        len += s[i].len();
    }

    for i in 0..=16-len {
        for p in s.iter().permutations(n+i) {
            
        }
        s.push(vec!['_']);
    }

}
