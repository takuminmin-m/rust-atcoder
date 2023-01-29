use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }

    a.sort();
    b.sort();
    let mut res = 0;
    for i in 0..n {
        res += (a[i]-b[i]).abs();
    }

    println!("{}", res);
}
