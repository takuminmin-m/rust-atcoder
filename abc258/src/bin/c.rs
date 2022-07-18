use proconio::{input, marker::Chars};
use std::mem::swap;

fn main() {
  input! {
    _n: usize,
    q: usize,
    mut s: Chars,
  }

  let mut queries = vec![(0, 0); q];
  for i in 0..q {
    input! {
      t: usize,
      x: usize,
    }
    queries[i] = (t, x);
  }


  for i in 0..q {
    if queries[i].0 == 1 {
      for _j in 0..queries[i].1 {
        let last_letter = s.pop();
        s.insert(0, last_letter.unwrap());
      }
    } else if queries[i].0 == 2 {
      println!("{}", s[queries[i].1 - 1]);
    }
  }
}
