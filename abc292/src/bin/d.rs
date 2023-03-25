use std::vec;

use petgraph::visit;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![Vec::<usize>::new(); n];
    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        g[u-1].push(v-1);
        g[v-1].push(u-1);
    }

    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }

        let mut visited = Vec::new();
        s(i, &g, &mut seen, &mut visited);
        let mut e_num = 0;
        for &j in visited.iter() {
            e_num += g[j].len();
        }
        e_num /= 2;
        if e_num != visited.len() {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn s(i: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, visited: &mut Vec<usize>) {
    seen[i] = true;
    visited.push(i);

    for j in 0..g[i].len() {
        if seen[g[i][j]] {
            continue;
        }
        s(g[i][j], g, seen, visited);
    }
}
