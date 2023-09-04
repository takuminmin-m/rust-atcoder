use std::println;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut seen = vec![vec![false; w]; h];
    search(&s, &mut seen, 0, 0, h, w, 0);
    if seen[h-1][w-1] {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn search(s: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>, a: usize, b: usize, h: usize, w: usize, l_: usize) {
    let l = l_ % 5;
    let snuke = "snuke".to_string();
    // println!("{} {} {} {} {}", a, b, h, w, l);
    if s[a][b]!=snuke.as_bytes()[l] as char {
        return;
    }
    if seen[a][b] {
        return;
    }
    seen[a][b] = true;


    if a==h && b==w {
        return
    }
    if a!=h-1 {
        search(s, seen, a+1, b, h, w, l+1);
    }
    if a!=0 {
        search(s, seen, a-1, b, h, w, l+1);
    }
    if b!=w-1 {
        search(s, seen, a, b+1, h, w, l+1);
    }
    if b!=0 {
        search(s, seen, a, b-1, h, w, l+1);
    }


}
