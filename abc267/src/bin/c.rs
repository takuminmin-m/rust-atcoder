use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
    }

    a.reverse();
    let mut max = 0;
    for i in 0..m {
        max += ((m-i) as i64) * a[i];
    }
    let mut prev = max;

    for i in 1..=(n-m) {
        let mut sum = prev;
        sum -= a[i-1] * m as i64;
        let new_sum: i64 = a[i..(i+m)].iter().sum();
        sum += new_sum;
        prev = sum;
        max = std::cmp::max(sum, max);
    }

    println!("{}", max);
}
