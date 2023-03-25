use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut j = 0;
    let mut c = 0;
    for i in 0..k {
        if a[j]==i {
            while j<a.len() && a[j]==i {
                j+=1;
            }
            c += 1;
            if c==k {
                println!("{}", i+1);
            }
            continue;
        } else {
            println!("{}", i);
            return;
        }
    }
}
