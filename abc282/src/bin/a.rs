use proconio::{ input, marker::Chars };

fn main() {
    input! {
        k: usize,
    }

    let s = &"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..k];
    println!("{}", s);
}
