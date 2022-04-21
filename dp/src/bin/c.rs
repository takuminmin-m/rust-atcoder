use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![[0, 0, 0]; n];

    for i in 0..n {
        input! {
            a_input: i32,
            b_input: i32,
            c_input: i32,
        }

        a[i][0] = a_input;
        a[i][1] = b_input;
        a[i][2] = c_input;
    }

    let mut dp = vec![[0, 0, 0]; n];

    for i in 0..3 {
        dp[0][i] = a[0][i];
    }

    for i in 1..(n) {
        for j in 0..3 {
            if j == 0 {
                dp[i][j] = if dp[i-1][1] < dp[i-1][2] { dp[i-1][2] } else { dp[i-1][1] } + a[i][j];
            } else if j == 1 {
                dp[i][j] = if dp[i-1][0] < dp[i-1][2] { dp[i-1][2] } else { dp[i-1][0] } + a[i][j];
            } else if j == 2 {
                dp[i][j] = if dp[i-1][0] < dp[i-1][1] { dp[i-1][1] } else { dp[i-1][0] } + a[i][j];
            }
        }
    }

    let mut ans = 0;
    for i in 0..3 {
        if ans < dp[n-1][i] { ans = dp[n-1][i]; }
    }

    println!("{}", ans);
}
