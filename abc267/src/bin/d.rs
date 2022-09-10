use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
    }

    let mut max = std::i64::MIN;
    for comb in a.iter().combinations(m) {
        let mut sum = 0;
        for i in 0..comb.len() {
            sum += ((i+1) as i64) * comb[i];
        }
        max = std::cmp::max(max, sum);
    }

    println!("{}", max);
}
