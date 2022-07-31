use proconio::{ input, marker::Chars };

fn main() {
    input! {
        q: usize,
    }

    let mut v = vec![String::new(); 0];
    for _ in 0..q {
        input! {
            s: String,
        }

        if s == "READ".to_string() {
            println!("{}", v[v.len()-1]);
            v.pop();
        } else {
            v.push(s);
        }
    }
}
