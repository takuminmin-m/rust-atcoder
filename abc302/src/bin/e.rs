use std::collections::HashSet;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut g = vec![HashSet::new(); n+1];
    let mut res = n;

    for _ in 0..q {
        input! {
            sig: i32,
        }

        if sig==1 {
            input! {
                u: usize,
                v: usize,
            }

            if g[u].len()==0 {
                res -= 1;
            }
            if g[v].len()==0 {
                res -= 1;
            }
            g[u].insert(v);
            g[v].insert(u);
        } else if sig==2 {
            input! {
                v: usize,
            }
            if g[v].len()!=0 {
                res += 1;
            }

            let a: Vec<&usize> = g[v].iter().collect();
            let ac: Vec<usize> = a.iter().map(|&x| *x).collect();
            for &u in ac.iter() {
                g[u].remove(&v);
                if g[u].len()==0 {
                    res += 1;
                }
            }
        }

        println!("{}", res);
    }

}
