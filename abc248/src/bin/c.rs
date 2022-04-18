use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k_: usize,
    }

    let mut dp = vec![vec![0; k_+1]; n+1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..k_ {
            for k in 0..(m+1) {
                if j + k <= k_ {
                    dp[i+1][j+k] += dp[i][j];
                    dp[i+1][j+k] %= 998244353;
                }
            }
        }
    }

    println!("{}", dp[n].iter().sum::<usize>() % 998244353);
}
