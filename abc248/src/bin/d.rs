use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    for i in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        }

        let a_sub = &a[(l-1)..(r)];
        let b = a_sub.iter().filter(|&&e| e==x).collect::<Vec<_>>();
        println!("{}", b.len());
    }
}
