use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    for _ in 0..n {
        input! {
            a: i64,
            b: i64,
        }

        println!("{}", a+b);
    }
}
