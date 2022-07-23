use proconio::{ input, marker::Chars };

fn main() {
  input! {
    chars: Chars,
  }

  if chars[0] == chars[1] && chars[1] == chars[2] {
    println!("-1");
    return;
  }

  if chars[0] == chars[1] {
    println!("{}", chars[2]);
    return;
  }

  if chars[1] == chars[2] {
    println!("{}", chars[0]);
    return;
  }

  if chars[0] == chars[2] {
    println!("{}", chars[1]);
    return;
  }

  println!("{}", chars[0]);
}
