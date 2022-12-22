use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    for i in 0..=n {
        println!("{}", n-i);
    }
}
