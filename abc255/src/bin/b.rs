use proconio::input;

fn get_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> f64 {
  return (((x1-x2).pow(2) + (y1-y2).pow(2)) as f64).sqrt();
}

fn main() {
  input! {
    n: usize,
    k: usize,
    a: [i64; k],
  }

  let mut x = vec![];
  let mut light_x = vec![];
  let mut y = vec![];
  let mut light_y = vec![];

  for i in 0..n {
    input! {
      x_elem: i64,
      y_elem: i64,
    }
    if a.iter().any(|e| (e - 1) == (i as i64) ) {
      light_x.push(x_elem);
      light_y.push(y_elem);
    } else {
      x.push(x_elem);
      y.push(y_elem);
    }
  }

  let mut max_dist = 0.;
  for i in 0..(n-k) {
    let mut min_dist = std::f64::INFINITY;
    for j in 0..(k) {
      let dist = get_distance(x[i], y[i], light_x[j], light_y[j]);
      // println!("{}", dist);
      if dist < min_dist {
        min_dist = dist;
      }
    }
    if min_dist > max_dist {
      max_dist = min_dist;
      // println!("{}", max_dist);
    }
  }

  println!("{}", max_dist)
}
