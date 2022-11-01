use std::vec;
use proconio::{ input, marker::Chars };
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        x: i32,
        y: i32,
        a: [i32; n],
    }

    let mut p = vec![vec![(0, 0); 0]; n+1];
    p[0].push((0, 0));
    p[1].push((a[0], 0));
    for i in 1..n {
        let ways = if i%2 == 0 {
            vec![(1, 0), (-1, 0)]
        } else {
            vec![(0, 1), (0, -1)]
        };

        for j in 0..p[i].len() {
            let mut new_places = vec![
                mov(p[i][j], ways[0], a[i]),
                mov(p[i][j], ways[1], a[i]),
            ];
            p[i+1].append(&mut new_places);
        }

        p[i+1].sort();
        p[i+1].dedup();
    }

    if p[n].iter().any(|&e| e==(x, y)) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn mov(place: (i32, i32), way: (i32, i32), d: i32) -> (i32, i32) {
    return (place.0 + way.0*d, place.1 + way.1*d);
}
