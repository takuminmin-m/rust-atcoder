use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }

    for bit_h in 0..(1<<h1) {
        let row_index = (0..h1).filter(|x| bit_h & (1 << x) != 0);

        for bit_w in 0..(1<<w1) {
            let col_index = (0..w1).filter(|x| bit_w & (1 << x) != 0);

            let mut new_a = vec![vec![0_usize; 0]; 0];
            row_index.clone().for_each(|i| {
                let mut new_row = vec![0_usize; 0];
                col_index.clone().for_each(|j| {
                    new_row.push(a[i][j]);
                });
                new_a.push(new_row);
            });

            if new_a == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
