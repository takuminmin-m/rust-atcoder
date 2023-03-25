use proconio::{ input, marker::Chars };
use num::abs;

fn main() {
    input! {
        r: usize,
        c: usize,
        mut b: [Chars; r],
    }

    for x in 0..c {
        for y in 0..r {
            if b[y][x]!='#' {
                continue;
            }
            let mut f = false;
            for p in 0..c {
                for q in 0..r {
                    if b[q][p]!='.' && b[q][p]!='#' {
                        let range = (b[q][p] as usize) - 48;
                        if dist(x, y, p, q) <= range {
                            b[y][x] = '.';
                            f = true;
                            break;
                        }
                    }
                }
                if f {
                    break;
                }
            }
        }
    }

    for y in 0..r {
        for x in 0..c {
            print!("{}",
                if b[y][x]=='#' {
                    b[y][x]
                } else {
                    '.'
                }
            )
        }
        println!();
    }
}

fn dist(x: usize, y: usize, p: usize, q: usize) -> usize {
    let x1 = x as i32;
    let x2 = p as i32;
    let y1 = y as i32;
    let y2 = q as i32;
    let res = abs(x1-x2) + abs(y1-y2);
    res as usize
}
