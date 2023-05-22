use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    for i in 0..(n-1) {
        if a[i] < a[i+1] {
            for j in a[i]..a[i+1] {
                print!("{} ", j);
            }
        } else {
            for j in (a[i+1]+1..=a[i]).rev() {
                print!("{} ", j);
            }
        }
    }
    println!("{}", a[n-1]);
}
