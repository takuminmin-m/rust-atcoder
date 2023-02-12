use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }

    let mut moti = vec![false; x+1];
    for e in b.iter() {
        moti[*e] = true;
    }

    let mut dp = vec![false; x+1];
    dp[0] = true;
    for i in 1..=x {
        if moti[i] {
            continue;
        }
        for e in a.iter().filter(|&&f| f <= i) {
            if dp[i-e] {
                dp[i] = true;
                break;
            }
        }
    }

    println!("{}", if dp[x] { "Yes" } else { "No" });
}
