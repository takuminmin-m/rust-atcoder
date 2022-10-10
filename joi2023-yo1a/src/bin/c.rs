use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut v = 1;
    let mut cnt = 0;

    for i in 0..n {
        if s[i] == 'R' {
            if v != 3 {
                v += 1;
            }

            if v == 3 {
                cnt += 1;
            }
        } else {
            if v != 1 {
                v -= 1;
            }
        }
    }

    println!("{}", cnt);
}
