use std::println;

use nalgebra::coordinates::X;
use ndarray::ShapeBuilder;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }


    let g = [
        ['#', '#', '#', '.', '-', '-', '-', '-', '-'],
        ['#', '#', '#', '.', '-', '-', '-', '-', '-'],
        ['#', '#', '#', '.', '-', '-', '-', '-', '-'],
        ['.', '.', '.', '.', '-', '-', '-', '-', '-'],
        ['-', '-', '-', '-', '-', '-', '-', '-', '-'],
        ['-', '-', '-', '-', '-', '.', '.', '.', '.'],
        ['-', '-', '-', '-', '-', '.', '#', '#', '#'],
        ['-', '-', '-', '-', '-', '.', '#', '#', '#'],
        ['-', '-', '-', '-', '-', '.', '#', '#', '#'],
    ];

    for i in 0..n-8 {
        for j in 0..m-8 {
            // 探索するコード
            let mut f = true;
            for x in 0..9 {
                for y in 0..9 {
                    if g[x][y] == '-' {
                        // print!("{} {}: ", i, j);
                        continue;
                    }

                    if g[x][y] != s[i+x][j+y] {
                        // println!("{} {} {} {} {} {}", i, j, x, y, g[x][y], s[i+x][i+y]);
                        f = false;
                        break;
                    }
                }
                if !f {
                    break;
                }
            }

            if f {
                println!("{} {}", i+1, j+1);
            }
        }
    }

}
