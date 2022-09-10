use proconio::{ input, marker::Chars };

fn main() {
    input! {
        string: Chars,
    }

    let mut s = Vec::<i32>::new();
    let atcoder = [(0, 'a'), (1, 't'), (2, 'c'), (3, 'o'), (4, 'd'), (5, 'e'), (6, 'r')];
    for c in string {
        let mut i = -1;
        for j in 0..7 {
            if c == atcoder[j].1 {
                i = atcoder[j].0;
            }
        }
        s.push(i);
    }

    let mut cnt = 0;
    for i in 0..(7-1) {
        for j in 0..(7-1) {
            if s[j] > s[j+1] {
                let c = s[j+1];
                s[j+1] = s[j];
                s[j] = c;
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
