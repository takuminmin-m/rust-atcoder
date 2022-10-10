use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let mut a: usize = 1;
    let mut b: usize = n;
    let mut c: usize = 1;
    let mut d: usize = n;

    let mut r_num = n / 2;
    for _i in 0..10 {
        let prev_b = b;
        b /= 2;
        println!("? {} {} {} {}", a, b, c, d);
        input! {
            res: usize,
        }
        if res == b-a+1 {
            a = b;
            b = prev_b;
        }


        let prev_d = d;
        d /= 2;
        println!("? {} {} {} {}", a, b, c, d);
        input! {
            res: usize,
        }
        if res == d-c+1 {
            c = d;
            d = prev_d;
        }
    }

    println!("{} {}", a, b);
}
