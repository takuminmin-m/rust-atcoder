use proconio::{ input, marker::Chars };

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a%b==0 {
        println!("{}", a/b);
    } else {
        println!("{}", a/b+1);
    }
}
