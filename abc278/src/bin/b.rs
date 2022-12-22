use proconio::{ input, marker::Chars };

fn main() {
    input! {
        mut h: i32,
        mut m: i32,
    }

    loop {
        if (h/10)*10 + m/10 < 24 && (h%10)*10 + m%10 < 60 {
            println!("{} {}", h, m);
            break;
        }
    m += 1;
    h += m/60;
    m %= 60;
    h %= 24;
    }
}
