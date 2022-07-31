use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![Vec::<usize>::new(); n];

    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        g[u-1].push(v-1);
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..(g[i].len()) {
            for k in 0..(g[g[i][j]].len()) {
                if g[i].iter().any(|e| *e==g[g[i][j]][k]) {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);
}
