use im_rc::HashMap;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        r_a: [usize; n],
        q: usize,
    }

    let mut c = 0_usize;
    let mut b = 0_usize;
    let mut a = Vec::<HashMap<usize, usize>>::new();
    for i in 0..n {
        let mut h = HashMap::<usize, usize>::new();
        h.insert(c, r_a[i]);
        a.push(h);
    }

    for _ in 0..q {
        input! {
            o: usize,
        }
        if o == 1 {
            input! {
                x: usize,
            }
            b = x;
            c += 1;
        } else if o == 2 {
            input! {
                i: usize,
                x: usize,
            }
            match a[i-1].get_mut(&c) {
                Some(v) => { *v += x; },
                None => { a[i-1].insert(c, b+x); }
            }
        } else if o == 3 {
            input! {
                i: usize,
            }
            match a[i-1].get(&c) {
                Some(v) => { println!("{}", v); },
                None => { println!("{}", b); }
            }
        }
    }
}
