use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [i64; n],
    q: usize,
  }

  a.sort();
  a.insert(0, -1000000000);

//   println!("{:?}", a);
  for _ in 0..q {
    input! {
      b: i64,
    }

    let mut l = 0;
    let mut r = n;
    let mut res: i64 = 1000000000;
    while r-l>1 {
      let m = (r+l)/2;
      let e = a[m];
      if e < b {
        l = m;
      } else {
        r = m;
      }
    }

    res = std::cmp::min((a[r]-b).abs(), (a[l]-b).abs());
    println!("{}", res);
  }
}
