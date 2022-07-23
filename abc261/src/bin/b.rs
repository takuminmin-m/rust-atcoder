use proconio::{ input, marker::Chars };

fn main() {
  input! {
    n: usize,
  }

  let mut a = vec![Vec::<char>::new(); n];
  for i in 0..n {
    input! {
      r: Chars,
    }
    a[i] = r;
  }

  for i in 0..n {
    for j in (i+1)..n {
      if a[i][j] == 'W' && a[j][i] != 'L' {
        println!("incorrect");
        return;
      }
      if a[i][j] == 'D' && a[j][i] != 'D' {
        println!("incorrect");
        return;
      }
      if a[i][j] == 'L' && a[j][i] != 'W' {
        println!("incorrect");
        return;
      }
    }
  }

  println!("correct");
}
