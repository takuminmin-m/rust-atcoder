use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let l = vec!["and", "not", "that", "the", "you"];
    for _ in 0..n {
        input! {
            w: String,
        }

        if l.iter().any(|&x| x==&w) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
