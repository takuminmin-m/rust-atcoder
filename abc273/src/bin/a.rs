use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: i32,
    }

    println!("{}", f(n));
}

fn f(k: i32)->i32 {
    if k==0 {
        return 1;
    }
    return k*f(k-1);
}
