use proconio::{ input, marker::Chars };

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut res = 1;
    for _ in 0..b {
        res *= a;
    }

    println!("{}", res);
}
