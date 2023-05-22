use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        d: i64,
        mut a: [i64; n],
        mut b: [i64; m],
    }

    a.sort();
    b.sort();
    let mut i = n-1;
    let mut j = m-1;

    let mut res = -1;

    loop {
        if (a[i]-b[j]).abs() <= d {
            res = std::cmp::max(res, a[i]+b[j]);
            break;
        }

        if i == 0 && j == 0 {
            break;
        }
        if i == 0 {
            j -= 1;
            continue;
        } else if j == 0 {
            i -= 1;
            continue;
        }

        if a[i] > b[j] {
            i -= 1;
        } else if a[i] < b[j] {
            j -= 1;
        } else {
            if a[i-1] > b[j-1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
    }

    println!("{}", res);
}
