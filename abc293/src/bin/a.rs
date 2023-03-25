use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }

    for i in 0..(s.len()/2) {
        print!("{}{}", s[2*i+1], s[2*i]);
    }
}
