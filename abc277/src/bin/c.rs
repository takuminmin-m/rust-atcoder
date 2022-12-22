use proconio::{ input, marker::Chars };
use std::collections::{ HashMap, HashSet };

fn main() {
    input! {
        n: usize,
    }
    let mut g: HashMap<usize, Vec<usize>> = HashMap::new();
    g.insert(1, Vec::<usize>::new());
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }

        match g.get_mut(&a) {
            Some(v) => v.push(b),
            None => { g.insert(a, vec![b]); },
        }
        match g.get_mut(&b) {
            Some(v) => v.push(a),
            None => { g.insert(b, vec![a]); },
        }
    }

    let mut seen: HashSet<usize> = HashSet::new();
    println!("{}", s(&g, &mut seen, 1));
}

fn s(g: &HashMap<usize, Vec<usize>>, seen: &mut HashSet<usize>, x: usize) -> usize {
    seen.insert(x);
    let mut max = x;
    for &e in g.get(&x).unwrap().iter() {
        if seen.get(&e).is_some() {
            continue;
        }
        max = std::cmp::max(max, s(g, seen, e));
    }

    return max;
}
