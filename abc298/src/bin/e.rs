use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
    }

    let mut dp_a = vec![(0, 1); n+11];
    let mut dp_b = vec![(0, 1); n+11];
    dp_a[a] = (1, 1);
    dp_b[b] = (1, 1);
    for i in 10..n+11 {
        let mut x_a = 1;
        let mut x_b = 1;
        let mut y_a = 0;
        let mut y_b = 0;
        for j in 0..p {
            x_a *= dp_a[i-1-j].1;
        }
        for j in 0..q {
            x_b *= dp_b[i-1-j].1;
        }
        for j in 0..p {
            y_a += dp_a[i-1-j].0*x_a/dp_a[i-1-j].1;
        }
        for j in 0..q {
            y_b += dp_b[i-1-j].0*x_b/dp_b[i-1-j].1;
        }
    }

    println!("{}", )
}
