use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut count = 0;

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if a[i] / a[j] == a[k] && a[i] % a[j] == 0 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
