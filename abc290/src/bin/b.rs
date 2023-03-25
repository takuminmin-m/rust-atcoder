use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut a = 0;
    for &c in s.iter() {
        if c=='o' && a < k {
            a += 1;
            print!("o");
        } else {
            print!("x");
        }
    }

    println!("");
}
