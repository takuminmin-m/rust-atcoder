use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut b = (1..=m).collect::<Vec<usize>>();
    for i in 0..(m-1) {
        let c = b[i];
        b[i] = b[i]
    }
}
