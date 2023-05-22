use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n]
    }

    for i in 0..c.len() {
        if a+b==c[i] {
            println!("{}", i+1);
            return;
        }
    }
}
