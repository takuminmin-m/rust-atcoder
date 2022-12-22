use std::f64::INFINITY;
use num_traits::Pow;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        a: u128,
        b: u128,
    }

    let mut c = 0;
    for i in (0..18).rev() {
        let mut prev = INFINITY;
        let mut best = 20;

        if i == 17 {
            for j in 0..10 {
                let g: u128 = 1 + (j as u128) * (10 as u128).pow(i as u32);
                let new = ((b*(g-1)) as f64) + (a as f64)/(g as f64).sqrt();
                if prev > new {
                    prev = new;
                    best = j as u128;
                } else {
                    break;
                }
            }
        } else {
            for j in 1..20 {
                let g: u128 = c + (j as u128) * (10 as u128).pow(i as u32);
                let new = ((b*(g-1)) as f64) + (a as f64)/(g as f64).sqrt();
                if prev > new {
                    prev = new;
                    best = j as u128;
                } else {
                    break;
                }
            }
        }
        if best == 0 || i==0 {
            c += best*10_u128.pow(i as u32);
        } else {
            c += (best-1)*10_u128.pow(i as u32);
        }
        // println!("{} {}", best, c);
    }

    println!("{}", ((b*(c-1)) as f64) + (a as f64)/(c as f64).sqrt());
}
