use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [usize; m],
    }

    let mut res = 0;
    for i in b.iter() {
        res += a[i-1];
    }

    println!("{}", res);
}
