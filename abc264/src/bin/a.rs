use proconio::{ input, marker::Chars };

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let s = "atcoder".to_string();
    println!("{}", &s[(l-1)..r]);
}
