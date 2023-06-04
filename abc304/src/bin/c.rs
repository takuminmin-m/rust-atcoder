use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        d_: i32,
        p: [(i32, i32); n],
    }
    let d = d_*d_;

    let mut g = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            if dist(p[i], p[j]) <= d {
                g[i].push(j);
            }
        }
    }

    let mut seen = vec![false; n];
    s(&g, &mut seen, 0);

    for e in seen.iter() {
        println!("{}", if *e { "Yes" } else { "No" });
    }
}

fn dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0-p2.0)*(p1.0-p2.0) + (p1.1-p2.1)*(p1.1-p2.1)
}

fn s(g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, p: usize) {
    seen[p] = true;

    for i in 0..g[p].len() {
        if seen[g[p][i]] {
            continue;
        }

        s(g, seen, g[p][i]);
    }
}
