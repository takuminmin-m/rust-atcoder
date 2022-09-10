use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    let mut cost = vec![0, 1, 2, 1];

    for i in 0..4 {
        let mut turned_s = s.clone();
        for j in 0..i {
            let mut new_s = vec![vec!['a'; n]; n];
            for h in 0..n {
                for w in 0..n {
                    new_s[w][n-h-1] = turned_s[h][w];
                }
            }
            turned_s = new_s;
        }

        for h in 0..n {
            for w in 0..n {
                if turned_s[h][w] != t[h][w] {
                    cost[i] += 1;
                }
            }
        }
    }

    let mut min = 99999999;
    for i in 0..4 {
        if cost[i] < min {
            min = cost[i];
        }
    }

    println!("{}", min);
}
