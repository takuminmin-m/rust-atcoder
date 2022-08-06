use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        p: [i32; n-1],
    }

    let mut i = 1;
    let mut anc: i32 = p[n-2];
    loop {
        if anc <= 1 {
            break;
        }
        anc = p[(anc-2) as usize];
        i += 1;
    }

    println!("{}", i);
}
