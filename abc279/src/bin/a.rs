use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }

    let mut res = 0;
    for &e in s.iter() {
        if e == 'v' {
            res += 1;
        } else {
            res += 2;
        }
    }

    println!("{}", res);
}
