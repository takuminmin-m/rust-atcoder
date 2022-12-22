use im_rc::HashMap;
use nalgebra::{coordinates::X, PermutationSequence};
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        mut g: [[i64; w]; h],
    }

    let mut colors = HashMap::<i64, usize>::new();
    let mut seen = vec![vec![false; w]; h];
    for x in 0..w {
        for y in 0..h {
            match colors.get_mut(&g[y][x]) {
                Some(v) => { *v+=1; },
                None => { colors.insert(g[y][x], 1); },
            };
        }
    }

    let mut res = colors.iter().map(|e| e.1).max().unwrap();
    for x in 0..w {
        for y in 0..h {
            if seen[y][x] {
                continue;
            }
            seen[y][x] = true;
            for (c, _) in colors.iter() {
                let mut new_seen = seen.clone();
                let prev_c = g[y][x];
                let mut new_g = g.clone();
                let mut seen2 = vec![vec![false; w]; h];
                let max = paint(&mut new_g, &mut new_seen, &mut seen2, x, y, w, h, *c);
                // println!("{} {} {} {}", max, x, y, c);
                res = std::cmp::max(max, res);
            }
        }
    }

    println!("{}", res);
}

fn paint(g: &mut Vec<Vec<i64>>, seen: &mut Vec<Vec<bool>>, seen2: &mut Vec<Vec<bool>>, x: usize, y: usize, w: usize, h: usize, c: i64)-> usize {
    let prev_c = g[y][x];
    g[y][x] = -1;
    let mut neighbors = vec![(0usize, 0usize); 0];

    if 0 != x {
        neighbors.push((x-1, y));
    }
    if w-1 != x {
        neighbors.push((x+1, y));
    }

    if 0 != y {
        neighbors.push((x, y-1));
    }
    if h-1 != y {
        neighbors.push((x, y+1));
    }

    // println!("{:?}", neighbors);
    let mut num = 1;

    for e in neighbors.iter() {
        if seen[e.1][e.0] || seen2[e.1][e.0] {
            continue;
        }
        if g[e.1][e.0] == prev_c {
            seen[e.1][e.0] = true;
            num += paint(g, seen, seen2,e.0, e.1, w, h, c);
        } else if g[e.1][e.0] == c {
            seen2[e.1][e.0] = true;
            num += count(g, seen2, x, y, w, h, c);
        }
    }

    return num;
}

fn count(g: &mut Vec<Vec<i64>>, seen: &mut Vec<Vec<bool>>, x: usize, y: usize, w: usize, h: usize, c: i64)-> usize {
    let mut neighbors = vec![(0usize, 0usize); 0];

    if 0 != x {
        neighbors.push((x-1, y));
    }
    if w-1 != x {
        neighbors.push((x+1, y));
    }

    if 0 != y {
        neighbors.push((x, y-1));
    }
    if h-1 != y {
        neighbors.push((x, y+1));
    }

    // println!("{:?}", neighbors);
    let mut num = 1;

    for e in neighbors.iter() {
        if g[e.1][e.0] != c || seen[e.1][e.0] {
            continue;
        }
        seen[e.1][e.0] = true;
        num += count(g, seen, e.0, e.1, w, h, c);
    }

    return num;
}
