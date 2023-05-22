use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    for perm in (0..n).permutations(n) {
        let mut f = true;
        for i in 0..n-1 {
            let mut dif = 0;
            for j in 0..m {
                if s[perm[i]][j] != s[perm[i+1]][j] {
                    dif += 1;
                    if dif > 1 {
                        f = false;
                        break;
                    }
                }
            }
            if !f {
                break;
            }
        }
        if f {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
