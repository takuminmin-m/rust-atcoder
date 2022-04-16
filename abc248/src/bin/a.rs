use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Chars,
    }

    let mut a = 0;

    for c in s {
        a += c as i32 - 48;
    }

    println!("{}", 45 - a)
}
