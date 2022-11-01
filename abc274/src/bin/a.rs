use proconio::{ input, marker::Chars };

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    if b == 0 {
        println!("0.000");
        return;
    }

    if a == b {
        println!("1.000");
        return;
    }

    let mut ans = b*10000 / a;

    if ans%10 >= 5 {
        ans += 10;
    }

    println!("0.{}", ans/10);
}
