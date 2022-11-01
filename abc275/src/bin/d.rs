use std::sync::mpsc::RecvTimeoutError;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: u64,
    }

    println!("{}", f(n));
}

fn f(x: u64) -> u64 {
    if x == 0 {
        return 1;
    }
    return f(x/2)+f(x/3);
}
