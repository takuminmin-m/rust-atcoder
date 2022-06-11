use proconio::input;

fn main() {
  input! {
    r: usize,
    c: usize,
    row1: [i32; 2],
    row2: [i32; 2],
  }

  let a = vec![row1, row2];

  println!("{}", a[r-1][c-1])
}
