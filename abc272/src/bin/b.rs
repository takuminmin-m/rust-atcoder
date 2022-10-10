use proconio::{ input, marker::Chars };
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut k = vec![0_usize; m];
    let mut x = vec![Vec::<usize>::new(); m];
    let mut people = vec![Vec::<usize>::new(); n];

    for i in 0..m {
        input! {
            km: usize,
            row: [usize; km],
        }
        k[i] = km;
        x[i] = row.clone();
        for p in 0..km {
            people[row[p]-1].push(i);
        }
    }

    if n == 2 && m == 1 {
        if k[0] == 2 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    for comb in (0..n).combinations(2) {
        for i in &people[*comb.first().unwrap()] {
            if people[comb[1]].iter().any(|e| e == i) {
                break;
            } else if people[*comb.first().unwrap()].last().unwrap() == i {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
