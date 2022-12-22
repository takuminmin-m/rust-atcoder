use std::vec;

use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: i64,
        a: [i64; n],
    }

    let mut dp = vec![vec![0_i64; k+1]; n+1];

    for i in 1..=n {
        for j in 1..=k {
            println!("{:?}", dp);
            if j != k {
                dp[i][j] = std::cmp::max(dp[i-1][j-1]+a[i-1], dp[i-1][j]);
                continue;
            }

            if dp[i][j-1]+a[i-1]%d == 0 {
                dp[i][j] = dp[i][j-1]+a[i-1];
            }
        }
    }


    match dp.iter().map(|e| e[k]).max() {
        Some(v) => { println!("{}", v); },
        None => { println!("-1"); },
    };
}
