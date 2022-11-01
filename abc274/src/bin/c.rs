use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = vec![(0_usize, -1); 2*n + 2];
    b[1] = (1, 0);

    for i in 0..n {
        let gen = b[a[i]].1 + 1;
        b[(i+1)*2].1 = gen;
        b[(i+1)*2+1].1 = gen;
    }

    for i in 1..(2*n+2) {
        println!("{}", b[i].1);
    }
}
