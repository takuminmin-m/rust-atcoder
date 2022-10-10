use proconio::{ input, marker::Chars };

fn main() {
    input! {
        x: i32,
        y: i32,
        z: i32,
    }

    if 0 < x {
        if 0 < y {
            if x < y {
                println!("{}", x.abs());
            } else {
                if 0 < z && z < y {
                    println!("{}", x.abs());
                } else if z < 0 {
                    println!("{}", x - 2*z);
                } else {
                    println!("-1");
                }
            }
        } else {
            println!("{}", x.abs());
        }
    } else {
        if y < 0 {
            if y < x {
                println!("{}", x.abs());
            } else {
                if z < 0 && y < z {
                    println!("{}", x.abs());
                } else if 0 < z {
                    println!("{}", 2*z - x);
                } else {
                    println!("-1");
                }
            }
        } else {
            println!("{}", x.abs());
        }
    }
}
