use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut b_w = vec![vec![false; w]; h];
    for i in 0..h {
        let mut prev = a[i][0];
        for j in 1..w {
            if prev==a[i][j] {
                b_w[i][j] = true;
                b_w[i][j-1] = true;
            }
            prev = a[i][j];
        }
    }
    // println!("{:?}", b_w);


    let mut dp = vec![vec![(0, vec![vec![0_usize; 0]; 0]); 0]; h];
    dp[0].push((0, a.clone()));
    dp[0].push((1, turn(&a, 0)));
    for i in 1..h {
        for j in 0..dp[i-1].len() {
            let new1 = dp[i-1][j].1.clone();
            let new2 = turn(&dp[i-1][j].1, i);
            let new1_b = is_isolated(&new1, i-1);
            let new2_b = is_isolated(&new2, i-1);
            if bool_sum(&new1_b, &b_w[i-1]) {
                let res = (dp[i-1][j].0, new1);
                dp[i].push(res);
            }
            if bool_sum(&new2_b, &b_w[i-1]) {
                let res = (dp[i-1][j].0+1, new2);
                dp[i].push(res);
            }
        }

        // println!("{:?}", dp);
    }

    let mut res = 999999999;
    for e in dp[h-1].iter() {
        res = std::cmp::min(res, e.0);
    }
    if res == 999999999 {
        println!("-1");
        return;
    }
    println!("{}", res);
}

fn turn(a: &Vec<Vec<usize>>, i: usize)->Vec<Vec<usize>> {
    let r: Vec<usize> = a[i].iter().map(|e| if *e==1 { 0 } else { 1 }).collect();
    let mut res = a.clone();
    res[i] = r;
    return res;
}

fn is_isolated(a: &Vec<Vec<usize>>, i: usize)->Vec<bool> {
    let mut res = vec![false; a[i].len()];
    for j in 0..a[i].len() {
        if i>0 {
            res[j] = res[j] || a[i-1][j] == a[i][j];
        }
        if i<a.len()-1 {
            res[j] = res[j] || a[i+1][j] == a[i][j];
        }
    }

    return res;
}

fn bool_sum(a: &Vec<bool>, b: &Vec<bool>)-> bool {
    let mut res = true;
    for i in 0..a.len() {
        res = res && (a[i] || b[i]);
    }

    return res;
}
