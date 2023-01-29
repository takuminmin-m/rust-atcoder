use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    // println!("here");
    let mut dp = vec![vec![false; x+1]; n+1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=x {
            for k in 0..=ab[i-1].1 {
                if j >= ab[i-1].0*k {
                    let l = j - ab[i-1].0*k;
                    if dp[i-1][l] {
                        dp[i][j] = true;
                        break;
                    }
                }
            }
        }
        // println!("{:?}", dp)    ;
    }

    println!("{}", if dp[n][x] { "Yes" } else { "No" });
}
