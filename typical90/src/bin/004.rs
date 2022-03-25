use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut a = vec![vec![0; w]; h];
    let mut b = vec![vec![0; w]; h];

    let mut row_sum = vec![0; h];
    let mut column_sum = vec![0; w];


    for i in &mut a {
        input! {
            row: [usize; w],
        }

        *i = row;
    }

    for i in 0..h {
        row_sum[i] = a[i].iter().sum();
    }

    for i in 0..w {
        for j in 0..h {
            column_sum[i] += a[j][i]
        }
    }

    for i in 0..h {
        for j in 0..w {
            b[i][j] = row_sum[i] + column_sum[j] - a[i][j];
        }
    }

    for i in b {
        for j in 0..(w-1) {
            print!("{} ", i[j]);
        }
        println!("{}", i[w-1]);
    }
}
