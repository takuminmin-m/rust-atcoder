use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut s: Chars,
    }

    let mut res = std::usize::MAX;
    for i in 0..n {
        let o1 = i * a;

        // println!("{:?}", s);

        let mut o2 = 0;
        for j in 0..(n/2) {
            if s[j] != s[n-j-1] {
                o2 += b;
            }
        }
        // println!("{}", o1);
        // println!("{}", o2);

        res = std::cmp::min(res, o1+o2);

        s.push(s[0]);
        s.remove(0);
    }

    println!("{}", res);
}
