use std::convert::TryInto;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![-1; n]; n];
    let mut mov_ = vec![(0, 0); 0];
    let mut mov = vec![(0_i32, 0_i32); 0];
    let a = [
        (1_i32, 1_i32),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]

    for x in 0..n {
        for y in 0..n {
            println!("{} {}", x, y);
            if dist(0, x as i64, 0, y as i64) == m {
                mov_.push((x, y));
            }
        }
    }

    for i in mov_ {
        a.iter().for_each(|&e|
            mov.push((i.0*e.0, i.1*e.1))
        );
    }

    println!("{:?}", mov);
}

fn dist(x1: i64, x2: i64, y1: i64, y2: i64) -> usize {
    return (((x1-x2)*(x1-x2) + (y1-y2)*(y1-y2)) as usize).try_into().unwrap();
}
