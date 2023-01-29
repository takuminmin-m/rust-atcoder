use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut flgs = vec![false; t.len()];
    let mut false_num = 0;
    for i in 0..t.len() {
        let c = s[s.len()-1-i];
        if c=='?' || t[t.len()-1-i]=='?' || c==t[t.len()-1-i] {
            flgs[i] = true;
        } else {
            false_num += 1;
        }
    }

    println!("{}", if flgs.iter().any(|e| !e) { "No" } else { "Yes" });

    flgs.reverse();
    for i in 0..t.len() {
        if t[i]=='?' || s[i]=='?' || t[i]==s[i] {
            if !flgs[i] {
                false_num -= 1;
            }
            flgs[i] = true;
        } else if flgs[i] {
            false_num += 1;
        }

        println!("{}", if false_num!=0 { "No" } else { "Yes" });
    }
}
