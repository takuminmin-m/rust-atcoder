use proconio::{input, marker::Chars, marker::Bytes};

fn main() {
    input! {
      n: usize,
    }

    let mut a_u8 = vec![Vec::<u8>::new(); n];
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
      input! {
        s: Bytes,
      }
      a_u8[i] = s;
      for j in 0..n {
        a[i][j] = a_u8[i][j] as i64 - 48;
      }
    }

    // println!("{:?}", a);


    let p: Vec::<i64> = vec![1, 1, 1, 0, 0, -1, -1, -1];
    let q: Vec::<i64> = vec![1, 0, -1, 1, -1, 1, 0, -1];
    let mut ans = 0;
    for i in 0..n {
      for j in 0..n {
        for k in 0..8 {
          let mut now = 0_i64;
          let mut x = i as i64;
          let mut y = j as i64;
          for l in 0..n {
            now *= 10;
            now += a[x as usize][y as usize];
            x += p[k as usize];
            y += q[k as usize];
            x %= n as i64;
            x += n as i64;
            y %= n as i64;
            y += n as i64;
            x %= n as i64;
            y %= n as i64;
          }
          ans = if ans < now { now } else { ans };
        }
      }
    }

    println!("{}", ans);
}
