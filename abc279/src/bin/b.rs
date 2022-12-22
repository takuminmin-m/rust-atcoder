use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
        t :Chars,
    }

    if s == t {
        println!("Yes");
        return;
    }
    if s.len() < t.len() {
        println!("No");
        return;
    }

    for i in 0..=(s.len()-t.len()) {
        if s[i..(i+t.len())] == t[0..t.len()] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
