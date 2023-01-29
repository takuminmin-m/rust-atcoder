use std::f32::consts::E;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        a: [usize; n],
    }

    let a1 = &a[0..(p-1)];
    let a2 = &a[(r-1)..s];
    let a3 = &a[q..(r-1)];
    let a4 = &a[(p-1)..q];
    let a5  = &a[s..n];

    let mut res = Vec::<usize>::new();

    for e in a1.iter() {
        res.push(*e);
    }
    for e in a2.iter() {
        res.push(*e);
    }
    for e in a3.iter() {
        res.push(*e);
    }
    for e in a4.iter() {
        res.push(*e);
    }
    for e in a5.iter() {
        res.push(*e);
    }

    for i in 0..res.len() {
        if i==res.len()-1 {
            println!("{}", res[i]);
        } else {
            print!("{} ", res[i]);
        }
    }
}
