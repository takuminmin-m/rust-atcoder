use std::collections::HashMap;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }

    let atcoder = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    let mut i = 0;
    let mut j = 0;

    s.sort();
    t.sort();
    let mut at_s = s.iter().filter(|&x| x==&'@').count() as i32;
    let mut at_t = t.iter().filter(|&x| x==&'@').count() as i32;
    s.retain(|&x| x!='@');
    t.retain(|&x| x!='@');

    // println!("{:?}", s);
    // println!("{:?}", t);

    while i<s.len() && j<t.len() {
        if s[i] != t[j] {
            let b1 = atcoder.iter().any(|&x| x==s[i]);
            let b2 = atcoder.iter().any(|&x| x==t[j]);
            if !b1&&!b2 {
                println!("No");
                return;
            }
            if b1 {
                i += 1;
                at_t -= 1;
            }
            if b2 {
                j += 1;
                at_s -= 1;
            }
            continue;
        }
        i += 1;
        j += 1;
    }

    if at_t < 0 || at_s < 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
