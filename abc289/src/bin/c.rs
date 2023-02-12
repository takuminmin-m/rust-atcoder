use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut t = vec![vec![false; m]; n];
    for i in 0..m {
        input! {
            c: usize,
        }
        for j in 0..c {
            input! {
                a: usize,
            }
            t[a-1][i] = true;
        }
    }

    // println!("{:?}", t);
    let mut res = 0;
    for b in 1..(1 << m) {
        let l: Vec<usize> = (0..m).filter(|x| (b & (1<<x)) != 0).collect();
        let mut f1 = true;
        for i in 0..n {
            let mut f2 = false;
            for j in l.iter() {
                if t[i][*j] {
                    f2 = true;
                    break;
                }
            }
            if !f2 {
                f1 = false;
                break;
            }
        }
        if f1 {
            res += 1;
        }
    }

    println!("{}", res);
}
