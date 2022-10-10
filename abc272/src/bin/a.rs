use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    println!("{}", a.iter().sum::<i32>());
}
