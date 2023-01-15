use proconio::{ input, marker::Chars };

fn main() {
    input! {
        s: Chars,
    }

    let mut a = vec![(0, vec![0; 0]); 1];
    let mut b = vec![false; 26];
    for i in 0..s.len() {
        let c = s[i];
        if c=='(' {
            a.push((i, vec![0; 0]));
        } else if c==')' {
            let q = a.pop().unwrap();
            for e in q.1.iter() {
                b[*e] = false;
            }
        } else {
            let char_num = alp2num(c);
            if b[char_num] {
                println!("No");
                return;
            }
            b[char_num] = true;
            let last_i = a.len()-1;
            a[last_i].1.push(char_num);
        }
    }

    println!("Yes");
}

fn alp2num(alp: char)->usize {
    return (alp as usize) - 97;
}
