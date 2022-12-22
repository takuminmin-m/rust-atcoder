use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![0usize; 0]; n];
    for _ in 0..m {
        input! {
            v1: usize,
            v2: usize,
        }
        g[v1-1].push(v2-1);
        g[v2-1].push(v1-1);
    }

    let mut color = vec![0_i32; n];
    let mut c = 1;
    for i in 0..n {
        if color[i]!=0 {
            continue;
        }
        let r = s(&mut g, &mut color, i, c);
        if !r {
            println!("0");
            return;
        }
        c += 2;
    }

    let mut res = (n*(n-1)/2) as i32;
    let mut v = vec![0; c as usize];
    for e in color.iter() {
        v[*e as usize] += 1;
    }
    for e in v.iter() {
        res -= e*(e-1)/2;
    }
    res -= m as i32;
    println!("{}", res);
}

fn s(g: &mut Vec<Vec<usize>>, color: &mut Vec<i32>, i: usize, c: i32)->bool {
    let mut res = true;
    color[i] = c;
    let mut new_c = 0;
    if c%2==1 {
        new_c = c+1;
    } else {
        new_c = c-1;
    }

    for j in 0..g[i].len() {
        let k = g[i][j];
        if color[k]==0 {
            let r = s(g, color, k, new_c);
            if !r {
                res = false;
            }
            continue;
        }
        if color[k]==new_c {
            continue;
        }
        res = false;
    }

    return res;
}
