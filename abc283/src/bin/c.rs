use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = s.len();
    let mut i = 0;
    while i < s.len()-1 {
        if s[i]=='0' && s[i+1]=='0' {
            i += 2;
            ans -= 1;
        } else {
            i += 1;
        }
    }

    println!("{}", ans);
}
