use std::sync::mpsc::RecvTimeoutError;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: u64,
    }

    let mut v = vec![(0_u64, 0_u64); 0];
    v.push((0, 1));
    println!("{}", f(&mut v, n));
}

fn f(v: &mut Vec<(u64, u64)>, x: u64) -> u64 {
    let mut a = v.iter().find(|&&e| e.0 == x);
    let res;
    if a.is_none() {
        res = (x, f(v, x/2)+f(v, x/3));
        v.push(res);
    } else {
        res = *a.unwrap();
    }

    return res.1;
}
