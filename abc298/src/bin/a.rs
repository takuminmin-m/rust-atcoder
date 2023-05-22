use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut f = false;
    for &c in s.iter() {
        if c=='x' {
            println!("No");
            return;
        } else if c=='o' {
            f = true;
        }
    }

    if f {
        println!("Yes");
    } else {
        println!("No");
    }
}
