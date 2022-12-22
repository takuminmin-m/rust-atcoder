use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    }

    let res = p.iter().position(|&e| e==x).unwrap();
    println!("{}", res+1);
}
