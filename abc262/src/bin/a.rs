use proconio::{ input, marker::Chars };

fn main() {
    input! {
        y: i32,
    }

    let m = y % 4;
    if m==0 {
        println!("{}", y + 2);
    } else if m==1 {
        println!("{}", y + 1);
    } else if m==2 {
        println!("{}", y);
    } else if m==3 {
        println!("{}", y + 3);
    }
}
