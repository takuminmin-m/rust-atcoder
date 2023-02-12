use std::vec;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
    }

    let mut dp = vec![vec![0; k]; n];
    for i in 0..n {
        dp[i] = vec![a[i]; k];
    }

    dp[0] = vec![0; k];
    for i in 1..n {
        dp[i][0] = 0;
        for j in 1..k {
            dp[i][j] -= dp[i-1][j-1];
            dp[i-1][j-1] = 0;
        }
    }

    println!("{:?}", dp);

    for _ in 0..q {
        input! {
            l_: usize,
            r_: usize,
        }
        let l = l_-1;
        let r = r_-1;


    }
}
