use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }

    let mut res = -1;
    for i in 0..s.len() {
        if s[s.len()-1-i] == 'a' {
            res = (s.len()-i) as i32;
            break;
        }
    }

    println!("{}", res);
}
