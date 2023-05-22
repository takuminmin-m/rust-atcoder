use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        mut a: [[i32; n]; n],
        b: [[i32; n]; n],
    }

    let mut ans = false;
    for t in 0..4 {
        let mut f = true;
        for i in 0..n {
            for j in 0..n {
                if a[i][j]==1 {
                    if b[i][j]!=1 {
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
        } else if t==3 {
            println!("No");
            return;
        }

        let mut c = a.clone();
        for i in 0..n {
            for j in 0..n {
                a[i][j] = c[n-1-j][i];
            }
        }
    }
}
