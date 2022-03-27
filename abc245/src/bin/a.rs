use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a < c {
        println!("Takahashi");
    } else if a == c && b <= d {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
