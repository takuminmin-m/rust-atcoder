use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    }

    for _ in 0..q {
        input! {
            num: usize,
        }
        if num == 1 {
            input! {
                k: usize,
                x: usize,
            }
            a[k-1] = x;
        } else {
            input! {
                k: usize,
            }
            println!("{}", a[k-1]);
        }
    }
}
