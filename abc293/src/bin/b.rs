use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut x = vec![false; n];
    for i in 0..n {
        if !x[i] {
            x[a[i]-1] = true;
        }
    }
    let mut r = Vec::new();
    for i in 0..n {
        if !x[i] {
            r.push(i+1);
        }
    }

    r.sort();
    println!("{}", r.len());
    for i in 0..(r.len()-1) {
        print!("{} ", r[i]);
    }
    println!("{}", r.last().unwrap());
}
