use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    for i in 1..n {
        let mut flg = false;
        for l in 0..(n-i) {
            if s[l]==s[l+i] {
                println!("{}", l);
                flg = true;
                break;
            }
        }
        if !flg {
            println!("{}", n-i);
        }
    }
}
