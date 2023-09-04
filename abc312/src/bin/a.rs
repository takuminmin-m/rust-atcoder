use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: String,
    }

    let t = [
        "ACE",
        "BDF",
        "CEG",
        "DFA",
        "EGB",
        "FAC",
        "GBD"
    ];
    for i in 0..t.len() {
        if s == t[i] {
            println!("Yes");
            return;
        };
    }

    println!("No");
}
