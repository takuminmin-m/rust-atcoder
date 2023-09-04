use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    }

    a.sort();
    b.sort();
    b.reverse();

    println!("{:?}", a);
    println!("{:?}", b);
    let mut j = 0;
    let mut prev_a = -1;
    for mut i in 0..a.len() {
        if prev_a == -1 {
            prev_a = a[i];
            continue;
        }
        if prev_a == a[i] {
            continue;
        } else {
            i -= 1;
            prev_a = -1;
        }

        while j-1 >= 0 && a[i] > b[j] {
            j-=1;
        }
        j += 1;
        if i >= j {
            println!("{}", a[i]);
            return;
        }
    }
    println!("{}", b[j]+1);

}
