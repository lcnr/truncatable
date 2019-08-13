mod is_prime;
mod miller_rabin;
mod aks;
mod poly;

use num_bigint::BigUint;
use num_traits::{One, Zero};

use std::io::{stdout, Write};

fn radix(x: &BigUint, radix: u32) -> String {
    x.to_radix_be(radix).into_iter().map(|c| std::char::from_digit(c as u32, radix).unwrap()).collect()
}

fn truncatable(base: u32, mut primality_check: impl FnMut(&BigUint) -> bool) {
    print!("Base {}:", base);
    
    let mut count = Vec::new();
    let mut offsets = vec![BigUint::one()];

    let values = 1..base;
    let mut stack = vec![(BigUint::zero(), values.clone())];

    let mut biggest = BigUint::zero();
    let mut len = stack.len();
    while let Some((v, ref mut iter)) = stack.last_mut() {
        if let Some(i) = iter.next() {
            if len > offsets.len() {
                offsets.push(offsets.last().unwrap() * base);
            }

            let mut next = &offsets[len - 1] * i;
            next += &*v;
            if primality_check(&next) {
                if len > count.len() {
                    count.push(1);
                } else {
                    count[len - 1] += 1;   
                }
                stack.push((next, values.clone()));
            }
        } else {
            if &*v > &biggest {
                let (v, _) = stack.pop().unwrap();
                biggest = v;
            } else {
                stack.pop();
            }

        }

        len = stack.len();
    }

    for i in count {
        print!(" {}", i);
    }
    println!("\nThe biggest truncatable prime in base {} with {} digits is {} == {}\n", base, offsets.len() - 1, radix(&biggest, base), radix(&biggest, 10));
}


fn main() {
    let aks = false;
    let check = if aks {
        aks::aks
    } else {
        is_prime::is_prime
    };

    use std::time::Instant;
    let now = Instant::now();
    for base in 3..10 {
        truncatable(base, check);
    }
    println!("{:?}", now.elapsed());
}