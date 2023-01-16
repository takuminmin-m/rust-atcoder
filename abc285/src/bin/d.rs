use std::collections::HashMap;
use std::collections::HashSet;

use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }

    let mut g = vec![300000_usize; 20000];
    let mut index = HashMap::<&String, usize>::new();
    let mut g_input = HashSet::<&String>::new();
    let mut counter = HashSet::<&String>::new();
    for (s, t) in st.iter() {
        let mut s_i = 0;
        let mut t_i = 0;
        match index.get(s) {
            Some(v) => {
                s_i = *v;
            },
            None => {
                index.insert(s, g.len());
            }
        }
        match index.get(t) {
            Some(v) => {
                t_i = *v;
            },
            None => {
                index.insert(t, g.len());
            }
        }

        g[s_i] = t_i;
    }

    let mut q = Vec::<&String>::new();
    for e in g.iter() {
        if !g_input.contains(key) {
            q.push(key);
        }
    }
    let mut deleted = 0;
    while q.len()!=0 {
        let key = q[0];
        q.remove(0);
        deleted += 1;
        match g.get(key) {
            Some(v) => {
                g_input.remove(v);
                q.push(v);
            },
            None => {}
        }
    }

    if counter.len()==deleted {
        println!("Yes");
    } else {
        print!("No");
    }
}
