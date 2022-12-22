use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u32; n],
    }

    for _ in 0..k {
        a.remove(0);
        a.push(0);
    }

    for i in 0..(n-1) {
        print!("{} ", a[i]);
    }

    println!("{}", a[n-1]);
}
