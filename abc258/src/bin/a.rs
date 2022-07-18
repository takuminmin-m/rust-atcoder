use proconio::input;

fn main() {
    input! {
      k: i32,
    }

    if k < 60 {
      println!("{}:{:02}", 21, k);
    } else {
      println!("{}:{:02}", 22, k-60);
    }
}
