use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut c = Vec::new();
    let mut res = Vec::new();

    for e in 1..=n {
        if a.iter().any(|x| *x==e) {
            c.push(e);
            continue;
        }
        res.push(e);
        for _ in 0..c.len() {
            res.push(c.pop().unwrap());
        }
    }

    for i in 0..(n-1) {
        print!("{} ", res[i]);
    }
    println!("{}", res[res.len()-1]);
}
