use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    let mut res = 0;
    for e in s.iter() {
        let rs = &e[3..6].to_vec();
        if t.iter().any(|x| x==rs) {
            res += 1;
        }
    }

    println!("{}", res);
}
