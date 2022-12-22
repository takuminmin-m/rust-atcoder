use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut res = 0;
    for e in s.iter().combinations(2) {
        let mut flg = false;
        for i in 0..m {
            if e[0][i]!='o' && e[1][i]!='o' {
                flg = true;
                break;
            }
        }
        if !flg {
            res += 1;
        }
    }

    println!("{}", res);
}
