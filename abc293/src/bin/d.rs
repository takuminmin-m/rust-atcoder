use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![0_usize; 0]; n*2];
    for _ in 0..m {
        input! {
            a_: usize,
            b: char,
            c_: usize,
            d: char,
        }
        let (a, c) = (a_-1, c_-1);
        let a_i = if b=='R' { a*2 } else { a*2+1 };
        let c_i = if d=='R' { c*2 } else { c*2+1 };
        g[a_i].push(c_i);
        g[c_i].push(a_i);
    }
    let mut y_sub = 0;
    for i in 0..n {
        g[i*2].push(i*2+1);
        g[i*2+1].push(i*2);
        if g[i*2].len()==1 && g[i*2+1].len()==1 {
            y_sub += 1;
        }
    }

    // println!("{:?}", g);

    let mut seen = vec![false; n*2];
    let mut x = 0;
    let mut y = 0;
    for i in 0..n*2 {
        if seen[i] {
            continue;
        }
        let mut seen2 = vec![false; n*2];
        if s(i, &g, &mut seen, &mut seen2) {
            x += 1;
        } else {
            y += 1;
        }
        println!("{} {}", x, y);

    }

    println!("{} {}", x-y_sub, y+y_sub);
}

fn s(i: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, seen2: &mut Vec<bool>) -> bool {
    seen[i] = true;
    seen2[i] = true;
    let mut f = true;
    for j in 0..g[i].len() {
        if seen2[g[i][j]] {
            continue;
        }
        f = false;
        if s(g[i][j], g, seen, seen2) {
            return true;
        }
    }
    f
}
