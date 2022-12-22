use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let max: u64 = *a.iter().max().unwrap();
    let min: u64 = *a.iter().min().unwrap();

    for &e in a.iter() {
        println!("{}", std::cmp::max(e-min, max-e));
    }
}
