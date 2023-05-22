use proconio::{ input, marker::Chars };
use num::integer::{ sqrt, cbrt };

fn main() {
    input! {
        n: usize,
    }

    let range_max = sqrt(n/12); // fix
    let primes = primes(range_max);
    let mut disable = 0;

    // println!("{:?}", primes);
    for i in 1..primes.len()-1 {
        let mut f1 = false;
        for j in i+1..primes.len() {
            let mut f2 = false;
            for k in j+1..primes.len()+1 {
                let num = primes[primes.len()-i]*primes[primes.len()-i] * primes[primes.len()-j] * primes[primes.len()-k]*primes[primes.len()-k];
                // println!("{}", num);
                if num > n as u128 {
                    disable += 1;
                    continue;
                }

                if primes[primes.len()-j] > primes[primes.len()-k]*primes[primes.len()-k] {
                    f2 = true;
                }
                if k==j+1 {
                    f1 = true;
                }
                break;
            }
            if f2 {
                break;
            }
        }
    }

    let res = (primes.len()-2)*(primes.len()-1)*primes.len()/6-disable;
    println!("{}",res);
}

fn primes(n: usize) -> Vec<u128> {
    let mut prime_f = vec![true; n+1];
    prime_f[0] = false;
    prime_f[1] = false;

    for i in 2..=sqrt(n)+1 {
        if prime_f[i] {
            for j in 2..=n/i {
                prime_f[j*i] = false;
            }
        }
    }

    let mut res = Vec::new();
    for i in 0..=n {
        if prime_f[i] {
            res.push(i as u128);
        }
    }

    res
}
