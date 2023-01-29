use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let mut c = 0;
    for _ in 0..n {
        input! {
            s: String,
        }
        if s == "For" {
            c += 1;
        }
    }

    println!("{}", if c > n/2 { "Yes" } else { "No" });
}
