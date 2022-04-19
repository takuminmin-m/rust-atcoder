use proconio::input;

const INF: i32 = std::f64::MAX as i32;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let mut dp = vec![INF; n];
    dp[0] = 0;

    dp[1] = (h[0] - h[1]).abs();

    for i in 2..n {
        if dp[i-1] + (h[i] - h[i-1]).abs() < dp[i-2] + (h[i] - h[i-2]).abs() {
            dp[i] = dp[i-1] + (h[i] - h[i-1]).abs();
        } else {
            dp[i] = dp[i-2] + (h[i] - h[i-2]).abs();
        }
    }

    println!("{}", dp[n-1]);
}
