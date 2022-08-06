use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    for comb in (1..(m+1)).combinations(n) {
        for i in 0..(comb.len()-1) {
            print!("{} ", comb[i]);
        }
        println!("{}", comb[comb.len()-1]);
    }
}
