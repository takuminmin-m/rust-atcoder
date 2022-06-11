use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut s = vec!["".to_string(); n];

    for i in &s {
        input! {
            s_: String,
        }
        s = s_;
    }

    println!("{:?}", s);

    let mut dp = vec![0; 26];

    for i in 0..(n) {
        for s_ in s[i].bytes() {
            let i_ = (s_ - 65) as usize;
            print!("{}", i_);
            dp[i_] += 1;
        }
    }
    println!("{:?}", dp);

    let mut count = 0;
    for i in dp {
        if i == k {
            count += 1;
        }
    }


    println!("{}", count);
}
