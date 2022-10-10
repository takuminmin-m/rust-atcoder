use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: i32,
    }

    if n%11 == 0 {
        println!("1");
        return;
    }

    println!("0");
}
