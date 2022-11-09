use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut res = vec![vec![0_usize; 0]; n+1];

    for e in ab {
        res[e.0].push(e.1);
        res[e.1].push(e.0);
    }

    for i in 1..=n {
        if res[i].len() == 0 {
            println!("0");
            continue;
        }
        print!("{} ", res[i].len());
        res[i].sort();
        for j in 0..(res[i].len()-1) {
            print!("{} ", res[i][j]);
        }
        println!("{}", res[i][res[i].len()-1]);
    }
}
