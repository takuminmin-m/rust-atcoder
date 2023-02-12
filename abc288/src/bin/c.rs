use std::{vec, hash::Hash};

use im_rc::HashSet;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![0_usize; 0]; n];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }

        g[a-1].push(b-1);
        g[b-1].push(a-1);
    }

    println!("{:?}", g);
    let mut seen = vec![false; n];
    let mut closed = Vec::<Vec<usize>>::new();
    let mut order = Vec::<usize>::new();
    s(&mut g, &mut seen, 0, 300000, &mut order, &mut closed);

    println!("{:?}", closed);
}

fn s(g: &mut Vec<Vec<usize>>, seen: &mut Vec<bool>, i: usize, prev_i: usize, order: &mut Vec<usize>, closed: &mut Vec<Vec<usize>>) {
    seen[i] = true;
    order.push(i);
    for j in 0..g[i].len() {
        if g[i][j]==prev_i {
            continue;
        }
        if seen[g[i][j]] {
            closed.push(order.clone());
            continue;
        }
        s(g, seen, g[i][j], i, order, closed);
    }
    order.pop();
}
