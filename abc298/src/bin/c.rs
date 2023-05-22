use std::collections::HashMap;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut nbox = HashMap::new();
    let mut ncard = HashMap::new();

    for _ in 0..q {
        input! {
            c: i32,
        }

        if c == 1 {
            input! {
                i: usize,
                j: usize,
            }

            ncard.entry(i).or_insert_with(|| vec![]).push(j);
            nbox.entry(j).or_insert_with(|| vec![]).push(i);
        } else if c == 2 {
            input! {
                i: usize,
            }

            nbox.get_mut(&i).unwrap().sort();
            let v = nbox.get(&i).unwrap();
            for j in 0..v.len()-1 {
                print!("{} ", v[j]);
            }
            println!("{}", v[v.len()-1]);
        } else if c == 3 {
            input! {
                i: usize,
            }

            ncard.get_mut(&i).unwrap().sort();
            ncard.get_mut(&i).unwrap().dedup();
            let v = ncard.get(&i).unwrap();
            for j in 0..v.len()-1 {
                print!("{} ", v[j]);
            }
            println!("{}", v[v.len()-1]);
        }
    }
}
