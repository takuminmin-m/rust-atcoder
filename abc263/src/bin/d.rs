use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        mut a: [i64; n],
    }
    a.push(0);

    let mut min = std::i64::MAX;
    for x in 0..n {
        for y in x..=n {
            let value: i64 = x as i64 *l + a[x..y].iter().sum::<i64>() + (n-y) as i64 *r;
            if min > value {
                min = value;
            }
        }
    }

    println!("{}", min);
}
