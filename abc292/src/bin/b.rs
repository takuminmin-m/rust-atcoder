use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut p = vec![0; n+1];

    for _ in 0..q {
        input! {
            t: i32,
            x: usize,
        }
        if t==1 {
            p[x] += 1;
        } else if t==2 {
            p[x] += 2;
        } else {
            println!("{}", if p[x]>=2 { "Yes" } else { "No" });
        }
    }
}
