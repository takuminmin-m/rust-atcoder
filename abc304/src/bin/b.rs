use std::{print, println};

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: Chars,
    }

    for i in 0..n.len() {
        if i > 2 {
            print!("0");
        } else {
            print!("{}", n[i]);
        }
    }

    println!("");
}
