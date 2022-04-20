use proconio::input;

const INF: i32 = std::f64::MAX as i32;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n],
    }

    let mut dp = vec![INF; n+1];
    dp[0] = 0;

    for i in 1..(n) {
        let mut min = INF;
        let l = if k < i { k } else { i };
        for j in 1..(l+1) {
            let value = dp[i-j] + (h[i] - h[i-j]).abs();
            if min > value {
                min = value;
            }
        }
        dp[i] = min;
    }

    println!("{}", dp[n-1]);
}
