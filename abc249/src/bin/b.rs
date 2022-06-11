use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut no = 0;

    for i in s.chars() {
        let found = &s.find(i);
        if Some(1) != *found && Some(0) != *found {
            println!("{:?}", s.find(i));
            print!("{:?}", i);
            println!("No");
            no = 1;
            break;
        }
    }

    if s.to_lowercase() == s || s.to_uppercase() == s {
        if no == 0 {
            println!("No");
        }
    } else {
        if no == 0 {
            println!("Yes");
        }
    }
}
