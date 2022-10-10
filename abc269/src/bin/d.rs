use std::vec;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let mut g = vec![vec![(0, 0); 2001]; 2001];
    for i in 0..n {
        g[(xy[i].0 + 1000) as usize][(xy[i].1 + 1000) as usize] = (1, 0);
    }

    let mut cnt = 0;
    for i in 0..n {
        if g[(xy[i].0 + 1000) as usize][(xy[i].1 + 1000) as usize].1 == 1 {
            continue;
        }
        f(&mut g, xy[i].0, xy[i].1);
        cnt += 1;
    }

    println!("{}", cnt);
}

fn f(g: &mut Vec<Vec<(i32, i32)>>, x: i32, y: i32) {
    if x < -1000 || 1000 < x || y < -1000 || 1000 < y || g[(x + 1000) as usize][(y + 1000) as usize].1 == 1 {
        return;
    }

    g[(x + 1000) as usize][(y + 1000) as usize].1 = 1;

    if g[(x + 1000) as usize][(y + 1000) as usize].0 == 0 {
        return;
    }

    let v = [
        (x-1, y-1),
        (x-1, y),
        (x, y-1),
        (x, y+1),
        (x+1, y),
        (x+1, y+1),
    ];

    for i in 0..6 {
        f(g, v[i].0, v[i].1);
    }
}
