use proconio::input;
use num::Float;
use std::f64::consts::PI;

fn main() {
  input! {
    a: f64,
    b: f64,
    d: f64,
  }

  let d_rad = (d / 180.0) * PI;
  let dis = Float::sqrt(a*a + b*b);

  let cos1 = a / dis;
  let sin1 = b / dis;
  let mut cos2 = cos1*d_rad.cos() - sin1*d_rad.sin();
  let mut sin2 = sin1*d_rad.cos() + cos1*d_rad.sin();
  if cos2.is_nan() {
    cos2 = 0.;
  }
  if sin2.is_nan() {
    sin2 = 0.;
  }

  println!("{} {}", cos2*dis, sin2*dis);
}
