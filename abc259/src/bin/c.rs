use proconio::{input, marker::Chars};

fn main() {
  input! {
    mut s: Chars,
    mut t: Chars,
  }

  let deduped_s = s.clone().dedup();
  let deduped_t = s.clone().dedup();
  if deduped_s != deduped_t {
    println!("No");
    return;
  }

  let mut s_num = vec![0; 0];
  let mut prev_letter: char = s[0];
  let mut count = 1;
  for i in 1..(s.len()-1) {
    if s[i] == prev_letter {
      count += 1;
    } else {
      s_num.push(count);
      count = 1;
      prev_letter = s[i];
    }
  }
  if s[s.len()-1] == prev_letter {
    s_num.push(count + 1);
  } else {
    s_num.push(count);
    s_num.push(1);
  }

  let mut t_num = vec![0; 0];
  prev_letter = t[0];
  count = 1;
  for i in 1..(t.len()-1) {
    if t[i] == prev_letter {
      count += 1;
    } else {
      t_num.push(count);
      count = 1;
      prev_letter = t[i];
    }
  }
  if t[t.len()-1] == prev_letter {
    t_num.push(count + 1);
  } else {
    t_num.push(count);
    t_num.push(1);
  }

  for i in 0..s_num.len() {
    if s_num[i] == t_num[i] {
      continue;
    } else if s_num[i] >= 2 && s_num[i] < t_num[i] {
      continue;
    } else {
      println!("No");
      return;
    }
  }

  println!("Yes");
}
