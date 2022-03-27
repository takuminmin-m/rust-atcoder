use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    for i in 0..2001 {
        if !(a.iter().any(|e| *e==i)) {
            println!("{}", i);
            break;
        }
    }
}
