use std::vec;

use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
    }

    let res = prev_perm(p, n);
    for i in 0..n-1 {
        print!("{} ", res[i]);
    }
    println!("{}", res[n-1]);
}

fn prev_perm(b: Vec<usize>, len: usize) -> Vec<usize> {
    let mut a = b.clone();
    let mut left = len - 2;
    while left >= 0 && a[left] <= a[left+1] {
        left -= 1;
    }
    let mut right = len - 1;
    while a[left] <= a[right] {
        right -= 1;
    }
    {
        let t = a[left];
        a[left] = a[right];
        a[right] = t;
    }
    left += 1;
    right = len - 1;
    while left < right {
        {
            let t = a[left];
            a[left] = a[right];
            a[right] = t;
        }
        left += 1;
        right -= 1;
    }

    return a;
}
