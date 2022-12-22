use proconio::{ input, marker::Chars };
use regex::internal::Char;

fn main() {
    input! {
        s: Chars,
    }

    if s.len() != 8 {
        println!("No");
        return;
    }

    let l = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let n = "0123456789";

    if !l.chars().any(|e| e==s[0]) {
        println!("No");
        return;
    }
    if !l.chars().any(|e| e==s[7]) {
        println!("No");
        return;
    }
    if s[1]=='0' || !n.chars().any(|e| e==s[1]) {
        println!("No");
        return;
    }

    for i in 2..7 {
        if !n.chars().any(|e| e==s[i]) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
