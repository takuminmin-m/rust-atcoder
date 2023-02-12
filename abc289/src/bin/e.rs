use proconio::{ input, marker::Chars };

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
        }

        let mut g = vec![vec![vec![0usize; 0]; n]; 2];
        let mut c = vec![0; n];
        for i in 0..n {
            input! {
                c_: usize,
            }
            c[i] = c_;
        }

        for _ in 0..m {
            input! {
                v1: usize,
                v2: usize,
            }

            if (c[v1-1]+c[v2-1]) % 2 == 0 {
                g[0][v1-1].push(v2-1);
                g[0][v2-1].push(v1-1);
            } else {
                g[1][v1-1].push(v2-1);
                g[1][v2-1].push(v1-1);
            }
        }

        let gd = c[0] == c[n-1];

    }
}

fn s(g: &mut Vec<Vec<Vec<usize>>>, seen1: &mut Vec<usize>, seen2: &mut Vec<usize>) {
    
}
