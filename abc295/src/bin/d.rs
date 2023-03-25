use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s_: Chars,
    }

    let mut s = Vec::new();
    for &e in s_.iter() {
        s.push(e as usize - 48);
    }

    let mut nums = vec![0; 10];
    let mut res = 0;
    for i in 1..=s.len() {
        for j in 0..i {
            nums[s[j]] += 1;
        }
        let sum: i32 = nums.iter().map(|x| x%2).sum();
        if sum==0 {
            // print!("{:?}  ", nums);
            // println!("{} {}", 1, i);
            res += 1;
        }
        for j in 0..(s.len()-i) {
            nums[s[j]] -= 1;
            nums[s[j+i]] += 1;
            let sum: i32 = nums.iter().map(|x| x%2).sum();
            if sum==0 {
                // print!("{:?}  ", nums);
                // println!("{} {}", j+2, j+i+1);
                res += 1;
            }
        }
        nums = vec![0; 10];
    }

    println!("{}", res);
}
