use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut s = vec![Vec::<char>::new(); h];
    for chars in &mut s {
        input! {
            row: Chars,
        }
        *chars = row;
    }

    let mut prev_c = s[0][0];
    for &c in &s[0][1..w] {
        if prev_c == c {
            println!("-1");
            return;
        } else {
            prev_c = c;
        }
    }

    println!("{}", w-1);
}
