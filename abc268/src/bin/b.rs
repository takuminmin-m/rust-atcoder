use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: String,
        t: String,
    }

    if s.len() > t.len() {
        println!("No");
        return;
    }

    if s == t[0..s.len()] {
        println!("Yes");
    } else {
        println!("No");
    }
}
