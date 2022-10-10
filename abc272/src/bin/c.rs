use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    if n == 2 {
        if (a[0] + a[1]) % 2 == 1 {
            println!("-1");
        } else {
            println!("{}", a[0] + a[1]);
        }
        return;
    }

    a.sort();
    let mut max_a = -1;
    let mut max_b = -1;

    let mut res_a = -1;
    let mut res_b = -1;

    for i in 1..=n {
        if a[n-i] % 2 == 1 {
            if max_b == -1 {
                max_b = a[n-i];
            } else if res_b == -1 {
                res_b = max_b + a[n-i];
                if res_a != -1 {
                    break;
                }
            }
        } else {
            if max_a == -1 {
                max_a = a[n-i];
            } else if res_a == -1 {
                res_a = max_a + a[n-i];
                if res_b != -1 {
                    break;
                }
            }
        }
    }

    println!("{}", std::cmp::max(res_a, res_b));
}
