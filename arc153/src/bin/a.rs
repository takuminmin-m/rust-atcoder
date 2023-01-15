use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    // 12 3 4 56 79 8
    let mut s = 100000;
    s += n-1;

    let s_string = s.to_string();
    let s_chars: Vec<char> = s_string.chars().collect();
    let i = vec![0_usize, 0, 1, 2, 3, 3, 4, 5, 4,];
    for e in i.iter() {
        print!("{}", s_chars[*e]);
    }

    println!("");
}
