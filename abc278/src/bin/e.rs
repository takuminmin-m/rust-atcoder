use proconio::{ input, marker::Chars };

fn main() {
    input! {
        hu: usize,
        wu: usize,
        n: usize,
        h: usize,
        w: usize,
    }

    let mut a = vec![vec![(0_usize, 0_usize); 0]; n];
    for i in 0..hu {
        for j in 0..wu {
            input! {
                e: usize,
            }
            a[e-1].push((i, j));
        }
    }

    for i in 0..=(hu-h) {
        for j in 0..=(wu-w) {
            let mut res = 0;
            for k in 0..n {
                if a[k].iter().any(|&e| i>e.0 || e.0>=(i+h) || j>e.1 || e.1>=(j+w)) {
                    res += 1;
                }
            }
            if j == wu-w {
                println!("{}", res);
            } else {
                print!("{} ", res);
            }
        }
    }
}
