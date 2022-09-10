use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: String,
    }

    let week = vec!["Saturday", "Friday", "Thursday", "Wednesday", "Tuesday", "Monday"];
    for i in 0..6 {
        if s == week[i].to_string() {
            println!("{}", i);
            return;
        }
    }
}
