use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut t: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();
    t %= sum;

    for i in 0..a.len() {
        if t < a[i] {
            println!("{} {}", i+1, t);
            return;
        } else {
            t -= a[i];
        }
    }
}
