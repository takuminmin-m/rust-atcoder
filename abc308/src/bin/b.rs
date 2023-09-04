use std::{collections::HashMap, println};

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [i32; m+1],
    }

    let mut map = HashMap::new();
    let base = p[0];
    for i in 0..m {
        map.insert(&d[i], p[i+1]);
    }

    let mut res = 0;
    for i in 0..n {
        res += match map.get(&c[i]) {
            Some(v) => v,
            None => &base,
        }
    }

    println!("{}", res);
}
