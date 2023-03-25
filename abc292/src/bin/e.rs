use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![0; 0]; n];
    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }

        g[u-1].push(v-1);
    }

    
}
