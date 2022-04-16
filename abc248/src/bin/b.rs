use proconio::input;

fn main() {
    input! {
        mut a: usize,
        b: usize,
        k: usize,
    }

    let mut c = 0;

    loop {
        if a >= b {
            break;
        }
        a *= k;
        c += 1;
    }

    println!("{}", c)
}
