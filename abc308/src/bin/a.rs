use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: [i32; 8],
    }


    let mut p = -1;
    for i in 0..8 {
        if s[i] < 100 || 675 < s[i] || s[i]%25 != 0 || p > s[i] {
            println!("No");
            return;
        }
        p = s[i];
    }

    println!("Yes");
}
