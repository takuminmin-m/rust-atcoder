use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
    }

    let mut s_v = vec![vec!['a', 'a']; 0];

    let fl = vec!['H', 'D', 'C', 'S'];
    let sl = vec!['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

    for i in 0..n {
        input! {
            s: Chars,
        }
        if !(fl.iter().any(|&e| e==s[0])) || !(sl.iter().any(|&e| e==s[1])) || s_v.iter().any(|e| e==&s) {
            println!("No");
            return;
        }
        s_v.push(s);
    }

    println!("Yes");
}
