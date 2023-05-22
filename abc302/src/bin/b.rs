use proconio::{ input, marker::Chars };

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let d: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    let string = "snuke";

    for i in 0..h {
        for j in 0..w {
            if s[i][j] != 's' {
                continue;
            }


            for (x, y) in d.iter() {
                let mut f = true;
                let mut res = vec![(i, j); 1];
                for k in 1..5 {
                    let a = (i as i32)+((x*k) as i32);
                    let b = (j as i32)+((y*k) as i32);
                    if !(a>=0 && a<h as i32 && b>=0 && b<w as i32) {
                        f = false;
                        break;
                    }

                    if s[a as usize][b as usize]!=string.chars().nth(k as usize).unwrap() {
                        f = false;
                        break;
                    }

                    res.push((a as usize, b as usize));
                }
                if f {
                    for (a, b) in res.iter() {
                        println!("{} {}", a+1, b+1);
                    }
                    return;
                }
            }
        }
    }
}
