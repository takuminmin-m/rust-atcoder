use std::slice::from_mut;

use im_rc::{HashSet, HashMap};
use num_integer::sqrt;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        k: u64,
    }

    let mut l = HashMap::<u64, u64>::new();
    let mut x = k;
    let mut res = 1;
    for i in 2..(sqrt(k)+2) {
        if x%i==0 {
            l.insert(i, 1);
            x /= i;
            res = i;
            while x%i==0 {
                *l.get_mut(&i).unwrap() += 1;
                x /= i;
            }
        }
    }
    if l.len() == 0 {
        println!("{}", k);
        return;
    }

    let mut mul = 1;
    for e in l.iter() {
        let mut c = 0;
        let mut d = 1;
        while e.1 > c {
            c += d;
            d += 1;
            if e.0.pow(c as u32) > res {
                mul *= e.0.pow((e.1-c) as u32);
                break;
            }
        }
    }

    println!("{}", res*mul);
}
