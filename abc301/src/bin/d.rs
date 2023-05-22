use std::slice::from_mut;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        mut s_: Chars,
        n_: usize,
    }

    s_.reverse();
    let mut s = vec![0; 64];
    for i in 0..s_.len() {
        if s_[i]=='1' {
            s[63-i] = 1;
        } else if s_[i]=='?' {
            s[63-i] = -1;
        }
    }

    let mut n = vec![0; 0];
    for i in 0..64 {
        if 1 << i == n_ & 1 << i {
            n.push(1);
        } else {
            n.push(0);
        }
    }
    n.reverse();

    // println!("{:?}", s);
    // println!("{:?}", n);

    let mut f = false;
    let mut f2 = false;
    let mut y: usize = 0;
    for i in 0..64 {
        if f {
            if s[i] == -1 {
                s[i] = 1;
            }
            continue;
        }
        if s[i]==-1 && n[i]==1 {
            f2 = true;
            // println!("!{}", i);
        }

        if s[i]==1 && n[i]==0 {
            if f2 {
                y += 1 << 63-i;
                continue;
            }
            // println!("{:?}", s);
            println!("-1");
            return;
        }
        if s[i]==-1 {
            // println!("{} {}", s[i], n[i]);
            s[i]=n[i];
        }
        if s[i]==0 && n[i]==1 {
            f = true;
        }
    }

    // println!("{:?}", s);
    let mut res = 0;
    for i in 0..64 {
        res += (s[i] as usize) << 63-i;
    }
    println!("{}", res - y);
}
