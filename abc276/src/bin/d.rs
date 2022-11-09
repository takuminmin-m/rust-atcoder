use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    let mut b = a.iter().map(|e| d(*e)).collect::<Vec<(u64, u64, u64)>>();
    let s = b[0].2;
    if b.iter().any(|x| x.2 != s) {
        println!("-1");
        return;
    }

    let mut min2: u64 = b.iter().map(|e| e.0).min().unwrap();
    let mut min3: u64 = b.iter().map(|e| e.1).min().unwrap();

    let res: u64 = b.iter().map(|e| e.0+e.1-min2-min3).sum();

    println!("{}", res);
}

fn d(a: u64) -> (u64, u64, u64) {
    let mut n = a;
    let mut res2 = 0;
    let mut res3 = 0;
    loop {
        if n%2 == 0 {
            n /= 2;
            res2 += 1;
        } else {
            break;
        }
    }
    loop {
        if n%3 == 0 {
            n /= 3;
            res3 += 1;
        } else {
            break;
        }
    }

    return (res2, res3, n);
}
