use std::vec;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let n = std::cmp::min(h, w);
    let mut res = vec![0; n+1];


    for i in 1..w-1 {
        for j in 1..h-1 {
            if c[j][i] == '.' {
                continue;
            }

            let d_max_w = std::cmp::min(i, w-i-1);
            let d_max_h = std::cmp::min(j, h-j-1);
            let d_max = std::cmp::min(d_max_h, d_max_w);
            for d in 1..=d_max {
                let c1 = c[j-d][i-d] == '#';
                let c2 = c[j-d][i+d] == '#';
                let c3 = c[j+d][i+d] == '#';
                let c4 = c[j+d][i-d] == '#';
                if c1 && c2 && c3 && c4 {
                    if d==d_max {
                        res[d] += 1;
                    }
                    continue;
                }
                res[d-1] += 1;
                break;
            }
        }
    }

    for i in 1..n {
        print!("{} ", res[i]);
    }

    println!("{}", res[n]);
}
