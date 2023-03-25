use std::vec;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let mut res = 0_u64;
    for i in 1..n {
        res += s(n-i) * s(i);
    }

    println!("{}", res);
}

fn s(n: usize) -> u64 {
    let mut res = 0;
    for i in 1..n {
        if n < i*i {
            break;
        }
        if n%i == 0 {
            if n/i == i {
                res += 1;
            } else {
                res += 2;
            }
        }
    }

    if res == 0 { 1 } else { res }
}
