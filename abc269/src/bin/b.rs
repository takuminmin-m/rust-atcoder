use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: [Chars; 10],
    }

    let mut a = 10;
    let mut b = 0;
    let mut c = 10;
    let mut d = 0;
    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                a = std::cmp::min(a, i);
                b = std::cmp::max(b, i);
                c = std::cmp::min(c, j);
                d = std::cmp::max(d, j);
            }
        }
    }

    println!("{} {}", a+1, b+1);
    println!("{} {}", c+1, d+1)
}
