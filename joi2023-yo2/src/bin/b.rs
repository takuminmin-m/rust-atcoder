use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        mut b: [u64; n],
        mut c: [u64; n],
        mut d: [u64; n],
    }

    let mut g = vec![vec![0_u64; n]; 4];
    g[0] = a;
    g[1] = b;
    g[2] = c;
    g[3] = d;

    g[0].sort();
    g[1].sort();
    g[2].sort();
    g[3].sort();

    let mut res = 99999999999_u64;
    for i in 0..n {
        for j in 0..4 {
            let min = g[j][i];
            let mut others = vec![0_u64; 0];
            let mut flg = false;
            for k in 0..4 {
                if k==j {
                    continue;
                }
                match g[k].iter().filter(|e| **e>=min).min() {
                    Some(v) => { others.push(*v); },
                    None => { flg = true; },
                }
            }

            if flg {
                continue;
            }
            let max = others.iter().max().unwrap();
            res = std::cmp::min(res, max-min);
        }
    }

    println!("{}", res);
}
