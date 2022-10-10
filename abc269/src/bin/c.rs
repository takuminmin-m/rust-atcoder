use proconio::{ input, marker::Chars };
use itertools::Itertools;
use std::iter::Iterator;

fn main() {
    input! {
        n: u64,
    }

    let mut a = n;
    let mut v = vec![0; 60];
    let mut flags = vec![0; 0];
    for i in 0..60 {
        let b = a % 2;
        if b == 1 {
            v[i] = b;
            flags.push(i);
        }
        a /= 2
    }

    for bit in 0..(1 << flags.len()) {
        let sub_list = (0..flags.len()).filter(|x| (bit & (1 << x)) != 0).map(|x| 2_u64.pow(flags[x as usize] as u32));
        let sum: u64 = sub_list.sum();
        println!("{}", sum);
    }
}
