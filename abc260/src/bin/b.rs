use proconio::{ input, marker::Chars };

fn main() {
  input! {
    n: usize,
    x: usize,
    y: usize,
    z: usize,
    a_list: [i32; n],
    b_list: [i32; n],
  }

  let mut a = vec![(0, 0); 0];
  let mut b = vec![(0, 0); 0];
  let mut c = vec![(0, 0); 0];
  for i in 0..n {
    a.push((a_list[i], i+1));
    b.push((b_list[i], i+1));
    c.push((a_list[i] + b_list[i], i+1));
  }

  a.sort_by(|a, b| (-a.0).partial_cmp(&(-b.0)).unwrap());
  let mut passed = vec![0; 0];
  for stu in a.drain(..x) {
    passed.push(stu.1);
    if let Some(remove_index) = b.iter().position(|s| s.1 == stu.1) {
      b.remove(remove_index);
    }
    if let Some(remove_index) = c.iter().position(|s| s.1 == stu.1) {
      c.remove(remove_index);
    }
  }

  b.sort_by(|a, b| (-a.0).partial_cmp(&(-b.0)).unwrap());
  for stu in b.drain(..y) {
    passed.push(stu.1);
    if let Some(remove_index) = a.iter().position(|s| s.1 == stu.1) {
      a.remove(remove_index);
    }
    if let Some(remove_index) = c.iter().position(|s| s.1 == stu.1) {
      c.remove(remove_index);
    }
  }

  c.sort_by(|a, b| (-a.0).partial_cmp(&(-b.0)).unwrap());
  for stu in c.drain(..z) {
    passed.push(stu.1);
  }

  passed.sort();
  for i in passed.iter() {
    println!("{}", i);
  }
}
