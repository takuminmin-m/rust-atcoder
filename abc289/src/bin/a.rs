use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }

    for &c in s.iter() {
        if c == '0' {
            print!("1");
        } else {
            print!("0");
        }
    }
    println!("");
}
