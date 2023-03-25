use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut res = 0;
    let mut mex = a[a.len()-1]+1;
    let mut b = vec![0_u128; 100000];

    let mut prev: i64 = -1;
    for &e_ in a.iter() {
        let e = e_ as i64;
        if e-prev > 1 {
            mex = (prev+1) as usize;
            break;
        }
        prev = e;
    }


    for i in 0..k {
        for j in 0..=mex {
            b[j] += 1;
        }
    }

    let sum: u128 = b.iter().sum();
    let p = fact(sum);
    let mut q = 1;
    for &e in b.iter() {
        if e==0 {
            break;
        }
        q *= fact(e);
    }

    println!("{} {}", p, q);

    println!("{}", (p/q) % 998244353);
}

fn fact(n: u128) -> u128 {
    let mut res = 1;
    for i in 1..=n {
        res *= i;
    }
    res
}
