use proconio::{ input, marker::Chars };

fn main() {
    input! {
        mut x: u64,
        k: usize,
    }

    for i in 0..k {
        // println!("{}", x);
        if (x / 10_u64.pow(i as u32)) % 10 >= 5 {
            x += 10_u64.pow((i+1) as u32);
        }
        x = (x/10_u64.pow((i+1) as u32)) * 10_u64.pow((i+1) as u32);
    }

    println!("{}", x);
}
