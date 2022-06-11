use proconio::input;

fn main(){
  input! {
    mut x: i64,
    a: i64,
    d: i64,
    n: i64,
  }

  x -= a;

  if d == 0 {
    println!("{}", x.abs());
    return;
  }
  let p = x / d;
  let q = x % d;
  let r = d * (n-1);

  if r <= x {
    if x - r < x {
      println!("{}", x - r);
    } else {
      println!("{}", x);
    }
  } else if q >= d - q {
    println!("{}", d - q);
  } else {
    println!("{}", q);
  }
}
