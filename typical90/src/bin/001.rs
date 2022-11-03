use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a:[usize; n],
    }


    let mut left = 0_usize;
    let mut right: usize = l-1;
    while right - left > 1 {
        let mid = left + (right-left)/2;
        let b = f(&a, n, l, k, mid);
        if b {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}

fn f(a: &Vec<usize>, n: usize, l: usize, k: usize, r: usize) -> bool {
    let mut cur_pos = 0_usize;
    let mut cut_cnt = 0_usize;
    for i in 0..n {
        if r <= a[i]-cur_pos {
            cur_pos = a[i];
            cut_cnt += 1;
        }
    }
    if r > l-cur_pos {
        cut_cnt -= 1;
    }

    return cut_cnt >= k;
}
