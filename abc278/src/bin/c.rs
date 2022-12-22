use im_rc::{HashMap, HashSet};
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut g = HashMap::<usize, HashSet<usize>>::new();
    for _ in 0..q {
        input! {
            t: usize,
            a: usize,
            b: usize,
        }
        if t == 1 {
            match g.get_mut(&a) {
                Some(v) => { v.insert(b); },
                None => {
                    let mut h = HashSet::<usize>::new();
                    h.insert(b);
                    g.insert(a, h);
                 }
            }
        } else if t == 2 {
            match g.get_mut(&a) {
                Some(v) => { v.remove(&b); },
                None => ()
            }
        } else {
            let mut f = true;
            match g.get(&a) {
                Some(v) => {
                    if !v.contains(&b) {
                        f = false;
                    }
                },
                None => { f = false }
            }
            match g.get(&b) {
                Some(v) => {
                    if !v.contains(&a) {
                        f = false;
                    }
                },
                None => { f = false }
            }
            if f {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
