use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input!{
        n: i32,
        m: i32,
        p: i32,
    }

    println!("{}", (n-m+p)/p);
}
