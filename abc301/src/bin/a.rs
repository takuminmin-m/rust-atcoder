use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut cnt_t = 0;
    let mut cnt_a = 0;
    for c in s.iter() {
        if *c=='T' {
            cnt_t += 1;
        } else {
            cnt_a += 1;
        }

        if n/2 + n%2 <= cnt_t {
            println!("T");
            return;
        } else if n/2 + n%2 <= cnt_a {
            println!("A");
            return;
        }
    }
}
