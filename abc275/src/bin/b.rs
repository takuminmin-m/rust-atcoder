use proconio::{ input, marker::Chars };

fn main() {
    input! {
        a: u128,
        b: u128,
        c: u128,
        d: u128,
        e: u128,
        f: u128,
    }

    let x = 998244353_u128;

    let a_a = a / x;
    let a_b = a % x;
    let b_a = b / x;
    let b_b = b % x;
    let c_a = c / x;
    let c_b = c % x;

    let d_a = d / x;
    let d_b = d % x;
    let e_a = e / x;
    let e_b = e % x;
    let f_a = f / x;
    let f_b = f % x;

    let left = a_b*b_b*c_b;
    let right = d_b*e_b*f_b;

    let res = if left > right {
        (left - right) % x
    } else {
        (x + right - left) % x
    };

    println!("{}", res);
}
