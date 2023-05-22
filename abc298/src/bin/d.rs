use proconio::{ input, marker::Chars };

fn main() {
    input! {
        q: usize,
    }

    let a = 998244353;
    let mut b = 1;
    let mut nums = vec![1_u64];
    for _ in 0..q {
        input! {
            c: i32,
        }

        if c == 1 {
            input! {
                x: u64,
            }

            nums.push(x);
            b +=
        } else if c == 2 {

        } else if c == 3 {

        }
    }
}
