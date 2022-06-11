use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        x: i32,
    }

    let mut taka = a*(x / (a + c))*b;
    let mut ao = d*(x / (d + f))*e;

    taka += if x%(a+c) < a { (x%(a+c))*b } else { a*b };
    ao += if x%(d+f) < a { (x%(d+f))*e } else { d*e };

    if taka < ao {
        println!("Aoki");
    } else if taka > ao {
        println!("Takahashi");
    } else {
        println!("Draw");
    }
}
