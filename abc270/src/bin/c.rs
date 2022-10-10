use std::vec;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        uv: [(usize, usize); n-1],
    }

    let mut g = vec![vec![0_usize; 0]; n+1];
    let mut seen = vec![false; n+1];
    let mut route = vec![0_usize; 0];
    let mut min_path = vec![0_usize, 0];
    let mut min = std::usize::MAX;
    for e in uv {
        g[e.0].push(e.1);
        g[e.1].push(e.0);
    }

    f(&mut g, &mut seen, x, &mut route, y, &mut min_path, &mut min);

    for i in 0..min_path.len()-1 {
        print!("{} ", min_path[i]);
    }
    println!("{}", min_path.last().unwrap());
}

fn f(g: &mut Vec<Vec<usize>>, seen: &mut Vec<bool>, a: usize, route: &mut Vec<usize>, y: usize, min_path: &mut Vec<usize>, min: &mut usize) {
    if seen[a] {
        return;
    }

    seen[a] = true;

    route.push(a);

    if a == y && *min > route.len() {
        *min = route.len();
        *min_path = route.clone();
        println!("{:?}", min_path);
    }

    for i in 0..g[a].len() {
        f(g, seen, g[a][i], route, y, min_path, min);
    }
}
