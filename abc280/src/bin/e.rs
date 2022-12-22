use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        p: u128,
    }

    let mut dp = vec![0; n+2];
    dp[0] = 1;
    for i in 1..=n {
        dp[i] += dp[i-1] * (100-p);
        dp[i+1] += dp[i-1] * p * 100;
    }

    println!("{:?}", dp);
}
