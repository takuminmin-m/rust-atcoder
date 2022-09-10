use std::vec;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }
    let col = vec![
        vec![6_usize],
        vec![3],
        vec![1, 7],
        vec![0, 4],
        vec![2, 8],
        vec![5],
        vec![9],
    ];

    if s[0] == '1' {
        println!("No");
        return;
    }

    let mut pin_num = vec![0; 7];
    for i in 0..7 {
        let mut cnt = 0;
        for j in 0..col[i].len() {
            if s[col[i][j]] == '1' {
                cnt += 1;
            }
        }
        pin_num[i] = cnt;
    }

    for i in 1..6 {
        if pin_num[i] == 0 && pin_num[i-1] != 0 {
            for j in (i+1)..7 {
                if pin_num[j] != 0 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
