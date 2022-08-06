use proconio::{ input, marker::Chars };

fn main() {
    input! {
        a: [u32; 5],
    }

    for i in 0..5 {
        let mut count = 0;
        for j in 0..5 {
            if a[i] == a[j] {
                count += 1;
            }
        }
        if count != 3 && count != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
