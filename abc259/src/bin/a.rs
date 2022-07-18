use proconio::input;

fn main() {
  input! {
    n: i32,
    m: i32,
    x: i32,
    t: i32,
    d: i32,
  }

  let mut ans = 0;
  if x <= m {
    ans = t;
  } else {
    ans = t - (x - m)*d;
  };

  println!("{:?}", ans);
}
