use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut g = vec![vec![0; 0]; n];
    for i in 0..n {
        input! {
            s: Chars,
        }
        for j in 0..n {
            if s[j]=='Y' {
                g[i].push(j);
            }
        }
    }

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            uo: usize,
            vo: usize
        }
        let u = uo - 1;
        let v = vo - 1;

        let mut que = vec![(u, 0, 0),];
        let mut q_seen = vec![vec![false; n]; 1];
        q_seen[0][u] = true;
        let mut min_o = -1;
        let mut res = 0;
        while que.len() != 0 {
            let c = que[0].0;
            let d = que[0].1;
            let p = que[0].2 + a[c];
            let seen = q_seen[0].clone();
            // println!("{:?}", que);
            // println!("{} {} {}", c, d, p);
            que.remove(0);
            q_seen.remove(0);

            if c == v && min_o == -1 {
                min_o = d;
            } else if min_o != -1 && min_o < d && c == v {
                break;
            }
            if c == v {
                res = std::cmp::max(res, p);
            }
            for &e in g[c].iter() {
                if seen[e] {
                    continue;
                }
                que.push((e, d+1, p));
                let mut new_seen = seen.clone();
                new_seen[c] = true;
                q_seen.push(new_seen);
            }
        }

        if min_o == -1 {
            println!("Impossible");
        } else {
            println!("{} {}", min_o, res);
        }
    }
}
