#![feature(core_intrinsics, try_from)]

mod is_prime;
mod miller_rabin;

use num_bigint::BigUint;

use crate::is_prime::is_prime;

use std::io::{stdout, Write};

fn radix(x: &BigUint, radix: u32) -> String {
    x.to_radix_be(radix).into_iter().map(|c| std::char::from_digit(c as u32, radix).unwrap()).collect()
}

fn truncatable(base: u32) {
    print!("Base {}: ", base);
    
    let mut numbers: Vec<_> = (2..base).filter_map(|n| {
        if (2..n).any(|i| n % i == 0) {
            None
        }
        else {
            Some(BigUint::from(n))
        }
    }).collect();

    let mut biggest_prime = numbers.last().unwrap().clone();

    let mut offset = BigUint::from(base);
    let mut digits = 1;
    while !numbers.is_empty() {
        print!("{}", numbers.len());
        stdout().flush().unwrap();

        numbers = numbers.iter().flat_map(|b| {
            (1..base).flat_map(|n| {
                let num = b + (&offset * n);
                if is_prime(&num) { 
                    Some(num) 
                }
                else { 
                    None 
                }
            }).collect::<Vec<_>>()
        }).collect();

        biggest_prime = numbers.last().map_or(biggest_prime, |num| num.clone());

        digits += 1;
        offset *= base;
        if !numbers.is_empty() {
            print!(", ");
        }
    }
    println!("\nThe biggest truncatable prime in base {} with {} digits is {}\n", base, digits, radix(&biggest_prime, base));
}


fn main() {
    let now = std::time::Instant::now();
    for base in 3..11 {
        truncatable(base);
    }
    println!("time spend: {:?}", std::time::Instant::now().duration_since(now));
}
