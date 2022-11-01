use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: [Chars; 9],
    }

    let mut res = 0;
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '.' {
                continue;
            }
            for a in 0..9 {
                for b in 1..9 {
                    if a > i || j+a+b >= 9 || i+b >= 9 {
                        continue;
                    }

                    if s[i+b][j+a]=='#' && s[i+b-a][j+a+b]=='#' && s[i-a][j+b] == '#' {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);
}
