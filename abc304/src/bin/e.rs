use std::collections::HashMap;
use std::collections::HashSet;
use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![(vec![0; 0], 10000000); n+1];
    for _ in 0..m {
        input! {
            v1: usize,
            v2: usize,
        }

        g[v1].0.push(v2);
        g[v2].0.push(v1);
    }

    let mut seen = HashSet::new();
    let mut p = 0;
    let mut isl_num = 0;
    while seen.len() < n {
        while seen.contains(&p) {
            p += 1;
        }
        s(&mut g, &mut seen, isl_num, p);
        isl_num += 1;
    }

    input! {
        k: usize,
    }
    let mut ban_list = HashSet::new();
    for _ in 0..k {
        input! {
            x: usize,
            y: usize,
        }
        let x_isl = g[x].1;
        let y_isl = g[y].1;
        ban_list.insert((x_isl, y_isl));
    }

    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
        }
        let a_isl = g[a].1;
        let b_isl = g[b].1;
        if ban_list.contains(&(a_isl, b_isl)) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}

fn s(g: &mut Vec<(Vec<usize>, usize)>, seen: &mut HashSet<usize>, isl_num: usize, p: usize) {
    seen.insert(p);
    g[p].1 = isl_num;

    for i in 0..g[p].0.len() {
        if seen.contains(&g[p].0[i]) {
            continue;
        }

        s(g, seen, isl_num, g[p].0[i]);
    }
}
