use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let mut ans = 0;
    for i in 0..n {
        ans += convert(s[i]) + ans*25;
    }

    println!("{}", ans);
}

fn convert(c: char) -> u64 {
    return ((c as u8) as u64) - 64
}
