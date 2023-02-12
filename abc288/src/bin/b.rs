use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut s = Vec::new();

    for _ in 0..k {
        input! {
            se: String,
        }
        s.push(se);
    }

    s.sort();
    for e in s.iter() {
        println!("{}", e);
    }
}
