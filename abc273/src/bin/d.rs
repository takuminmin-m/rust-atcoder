use std::sync::Arc;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: i64,
        cs: i64,
        n: usize,
    }

    let mut wall = vec![vec![0; w+1];h+1];
    for i in 0..n {
        input! {
            r: usize,
            c: usize,
        }
        wall[r][c] = 1;
    }

    input! {
        q: usize,
        dl: [(char, i64); q],
    }


    let mut p = (rs, cs);
    for e in dl {
        let mut mov: (i64, i64) = if e.0 == 'L' {
            (0, -1)
        } else if e.0 == 'R' {
            (0, 1)
        } else if e.0 == 'U' {
            (-1, 0)
        } else {
            (1, 0)
        };

        mov.0 *= e.1;
        mov.1 *= e.1;

        let mut moved_place = p_plus(p, mov);

        if moved_place.0 <= 0 {
            moved_place.0 = 1;
        } else if moved_place.0 > (h as i64) {
            moved_place.0 = h as i64;
        }
        if moved_place.1 <= 0 {
            moved_place.1 = 1;
        } else if moved_place.1 > (w as i64) {
            moved_place.1 = w as i64;
        }

        if p.0 == moved_place.0 {
            if p.1 < moved_place.1 {
                let res = wall[p.0 as usize][(p.1 as usize)..=(moved_place.1 as usize)].iter().position(|&e| e==1);
                if res != None {
                    moved_place.1 = res.unwrap() as i64 + p.1 - 1;
                }
            } else {
                let res = wall[p.0 as usize][(moved_place.1 as usize)..=(p.1 as usize)].iter().rposition(|&e| e==1);
                if res != None {
                    moved_place.1 = res.unwrap() as i64 + moved_place.1 + 1;
                }
            };
        } else {
            if p.0 < moved_place.0 {
                let res = wall[(p.0 as usize)..=(moved_place.0 as usize)].iter().position(|e| e[p.1 as usize]==1);
                if res != None {
                    moved_place.0 = res.unwrap() as i64 + p.0 - 1;
                }
            } else {
                let res = wall[(moved_place.0 as usize)..=(p.0 as usize)].iter().rposition(|e| e[p.1 as usize]==1);
                if res != None {
                    moved_place.0 = res.unwrap() as i64 + moved_place.0 + 1;
                }
            };
        }

        p = moved_place;
        println!("{} {}", p.0, p.1);
    }
}

fn p_plus(p1: (i64, i64), p2: (i64, i64)) -> (i64, i64) {
    return (p1.0+p2.0, p1.1+p2.1);
}
