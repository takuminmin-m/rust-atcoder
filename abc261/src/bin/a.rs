use proconio::{ input, marker::Chars };

fn main() {
  input! {
    l1: usize,
    r1: usize,
    l2: usize,
    r2: usize,
  }

  let left = if l1 < l2 { l2 } else { l1 };
  let right = if r1 < r2 { r1 } else { r2 };

  if left >= right {
    println!("0");
    return;
  }
  println!("{}", right - left);
}
