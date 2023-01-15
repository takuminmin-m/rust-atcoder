use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        mut g: [Chars; h],
        q: usize,
    }

    let mut w_i :Vec<usize> = (0..w).collect();
    let mut h_i :Vec<usize> = (0..h).collect();
    for i in 0..q {
        input! {
            a: usize,
            b: usize,
        }
        let old_w_i = w_i.clone();
        let old_h_i = h_i.clone();
        for i in 0..b {
            w_i[i] = old_w_i[b-1-i];
        }
        for i in b..w {
            w_i[i] = old_w_i[w-i+b-1];
        }
        for i in 0..a {
            h_i[i] = old_h_i[a-1-i];
        }
        for i in a..h {
            h_i[i] = old_h_i[h-i+a-1];
        }
    }

    for i in h_i.iter() {
        for j in w_i.iter() {
            print!("{}", g[*i][*j]);
        }
        println!("");
    }
}
