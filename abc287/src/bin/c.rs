use itertools::Itertools;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![0; 0]; n];
    for _ in 0..m {
        input! {
            v1: usize,
            v2: usize,
        }
        g[v1-1].push(v2-1);
        g[v2-1].push(v1-1);
    }

    let mut start;
    // println!("{:?}", g);
    match g.iter().find_position(|v| v.len()==1) {
        Some(v) => { start = v.0 },
        None => { println!("No"); return; }
    }

    let mut seen = vec![false; n];
    s(&g, &mut seen, start);

    if seen.iter().any(|e| !e) {
        println!("No");
    } else {
        println!("Yes");
    }
}

fn s(g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, p: usize) {
    // println!("{:?}", seen);
    seen[p] = true;
    for e in g[p].iter() {
        if seen[*e] {
            continue;
        }
        s(g, seen, *e);
    }
}
