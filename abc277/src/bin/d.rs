use proconio::{ input, marker::Chars };
use std::{collections::{ HashMap, HashSet }, f32::consts::E};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();
    let sum: usize = a.iter().sum();
    let mut v = vec![0_usize];
    for i in 0..n {
        if a[(i+n-1)%n] == (a[i]+n-1)%n {
            continue;
        }
        let mut max = 0;
        let mut prev = a[i];
        for j in 0..n {
            let e =  a[(i+j)%n];
            if e != prev && e != prev+1 {
                break;
            }
            max += e;
            prev = e;
        }
        v.push(max);
    }

    println!("{}", sum-v.iter().max().unwrap());
}
