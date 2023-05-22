use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h],
        b: [Chars; h],
    }

    for s in 0..w {
        for t in 0..h {
            let mut f = true;
            for i in 0..w {
                for j in 0..h {
                    if b[j][i]!=a[(j+t)%h][(i+s)%w] {
                        f = false;
                        break;
                    }
                }
                if !f {
                    break;
                }
            }
            if f {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
