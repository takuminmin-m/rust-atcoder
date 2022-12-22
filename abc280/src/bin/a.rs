use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut res = 0;
    for e in s.iter() {
        for i in 0..w {
            if e[i]=='#' {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
