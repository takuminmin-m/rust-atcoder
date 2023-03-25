use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    }

    let mut cur = (0, 0);
    let mut a_flat = Vec::new();
    for r in a.iter() {
        for e in r.iter() {
            a_flat.push(e);
        }
    }

    for i in 0..h*w {
        for j in 0..h*w {
            if a_flat[i] == a_flat[j] {
                
            }
        }
    }
}
