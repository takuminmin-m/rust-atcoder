use std::vec;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut s: Chars,
        }

        let mut dp = vec![false; n];
        let mut q = Vec::new();
        let mut res = 0;
        let mut f = false;

        for i in 0..n {
            if s[n-i-1]=='1' {
                q.push(n-i-1);
            }
        }

        while q.len() != 0 {
            let i = q.pop().unwrap();
            if q.len()==0 {
                f = true;
                break;
            }
            let j = q[q.len()-1];
            let mut ji = q.len()-1;
            if j - i == 1 {
                if q.len()==1 {
                    f = true;
                    break;
                }
                ji = q.len()-2;
            }

            q.remove(ji);
            res += 1;
        }

        if f {
            println!("-1 {} {:?}", res, q);
            continue;
        }

        println!("{}", res);
    }
}
