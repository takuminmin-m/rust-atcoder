use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut f = false;
    for &e in s.iter() {
        if e=='n' {
            f = true;
        } else if e=='a' && f {
            print!("y");
            f = false;
        } else {
            f = false;
        }
        print!("{}", e);
    }

    println!("");
}
