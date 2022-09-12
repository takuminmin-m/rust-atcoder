use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        p: [i32; n],
    }

    let mut a = vec![0; n];
    for i_ in 0..n {
        let i: i32 = i_ as i32;
        a[p[i_] as usize] = if p[i_] - i >= 0 {
            p[i_] - i
        } else {
            (n as i32) + p[i_] - i
        };
    }

    // println!("{:?}", a);

    let mut d = vec![0; n];
    for i in 0..n {
        d[a[i] as usize] += 1;
    }

    let mut max = -1;
    for i in 0..(n-2) {
        let sum = d[i] + d[i+1] + d[i+2];
        max = std::cmp::max(sum, max);
    }

    let sum1 = d[n-2] + d[n-1] + d[0];
    let sum2 = d[n-1] + d[0] + d[1];
    max = std::cmp::max(max, std::cmp::max(sum1, sum2));

    println!("{}", max);
}
