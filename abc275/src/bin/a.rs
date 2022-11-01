use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut max: i64 = 0;
    let mut res = 0;

    for i in 0..n {
        if h[i] > max {
            max = h[i];
            res = i;
        }
    }

    println!("{}", res + 1);
}
