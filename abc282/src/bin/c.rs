use proconio::{ input, marker::Chars };

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut flg = true;
    for mut e in s.iter() {
        if *e=='"' {
            flg = !flg;
        }

        if flg && *e==',' {
            e = &'.';
        }

        print!("{}", e);
    }

    println!("");
}
