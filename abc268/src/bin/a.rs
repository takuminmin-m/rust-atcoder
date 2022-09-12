use proconio::{ input, marker::Chars };

fn main() {
    input! {
        mut v: [i32; 5],
    }

    v.sort();
    v.dedup();

    println!("{}", v.len());
}
