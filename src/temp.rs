use num_bigint::BigUint;
use num_traits::Zero;

mod prime_book;

use crate::prime_book::PrimeBook;

fn radix(x: &BigUint, radix: u32) -> String {
    x.to_radix_be(radix).into_iter().map(|c| std::char::from_digit(c as u32, radix).unwrap()).collect()
}

fn truncatable(base: u32, book: &mut PrimeBook) {
    println!("starting with base {}", base);
    println!("there are currently {} primes in the book", book.len());
    let mut numbers: Vec<_> = (2..base).filter_map(|n| {
        let num = BigUint::from(n);
        if book.is_prime(&num) { Some(num) } else { None }
    }).collect();

    let mut offset = BigUint::from(base);
    let mut digits = 1;
    while !numbers.is_empty() {
        println!("There are {:4} numbers with {} digits", numbers.len(), digits);

        let new: Vec<_> = (1..base).flat_map(|n| {
            numbers.iter().filter_map(|b| {
                let num = b + (&offset * n);
                if book.is_prime(&num) { Some(num) } else { None }
            }).collect::<Vec<_>>()
        }).collect();

        if new.is_empty() {
            println!("The biggest truncatable prime in base {} with {} digits is {}", base, digits, radix(numbers.last().unwrap(), base));
        }

        digits += 1;
        numbers = new;
        offset *= base;
    }

}


fn main() {
    let mut book = PrimeBook::new();
    for base in 2.. {
        truncatable(base, &mut book);
    }
}
