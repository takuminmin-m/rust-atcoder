use proconio::{ input, marker::Chars };

fn main() {
  input! {
    n: usize,
  }

  let mut s = vec![(Vec::<char>::new(), false); n];
  for i in 0..n {
    input! {
      r: Chars,
    }
    s[i] = (r, false);
  }

  for i in 0..n {
    if s[i].1 {
      continue;
    }

    s[i].1 = true;

    let mut count = 0;
    for j in (i+1)..n {
      if s[i].0 == s[j].0 {
        count += 1;
        let stri: String = count.to_string();
        let cs: Vec<char> = stri.chars().collect();
        s[j].0.push('(');
        for k in 0..(cs.len()) {
          s[j].0.push(cs[k]);
        }
        s[j].0.push(')');
        s[j].1 = true;
      }
    }
  }

  for i in 0..n {
    for j in 0..(s[i].0.len()) {
      print!("{}", s[i].0[j]);
    }
    println!("");
  }
}
