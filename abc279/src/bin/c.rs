use im_rc::HashMap;
use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        s_: [Chars; h],
        t_: [Chars; h],
    }

    let mut s = HashMap::<Vec<char>, u32>::new();
    for i in 0..w {
        let col = s_.iter().map(|e| e[i]).collect::<Vec<char>>();
        match s.get_mut(&col) {
            Some(v) => *v += 1,
            None => {
                s.insert(col, 1);
            }
        }
    }

    for i in 0..w {
        let col = t_.iter().map(|e| e[i]).collect::<Vec<char>>();
        match s.get_mut(&col) {
            Some(v) => {
                if *v == 0 {
                    println!("No");
                    return;
                }
                *v -= 1;
            },
            None => {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
