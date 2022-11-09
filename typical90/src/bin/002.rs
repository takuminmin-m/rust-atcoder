use nalgebra::coordinates::X;
use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 1 {
        return;
    }

    let mut s = Vec::<Vec<char>>::new();

    for i in 0..(1 << n) {
        let mut new_s: Vec<char> = (0..n).map(|e| if i & (1 << e) != 0 { ')' } else { '(' }).collect();
        new_s.reverse();
        // println!("{:?}", new_s);
        s.push(new_s);
    }

    for p in s.iter() {
        let mut x = 0;
        let mut f = true;
        for e in p {
            if *e == '(' {
                x += 1;
            } else {
                x -= 1;
                if x < 0 {
                    f = false;
                    break;
                }
            }
        }

        if f && x == 0 {
            p.iter().for_each(|e| print!("{}", e));
            println!("");
        }
    }
}
