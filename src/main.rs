mod aks;
mod is_prime;
mod miller_rabin;
mod poly;

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn radix(x: &BigUint, radix: u32) -> String {
    x.to_radix_be(radix)
        .into_iter()
        .map(|c| std::char::from_digit(c as u32, radix).unwrap())
        .collect()
}

fn truncatable(base: u32, mut primality_check: impl FnMut(&BigUint) -> bool) {
    print!("Base {}: ", base);

    let mut count = [0; 100];
    let mut offsets = vec![BigUint::one()];
    for _ in 0..100 {
        offsets.push(offsets.last().unwrap() * base);
    }

    let values = 1..base;

    let mut biggest = BigUint::zero();
    let mut biggest_pos = 0;
    let mut pos = 0;

    let mut stack = vec![(BigUint::zero(), values.clone())];
    while let Some((v, ref mut iter)) = stack.last_mut() {
        if let Some(i) = iter.next() {
            let mut next = &offsets[pos] * i;
            next += &*v;
            if primality_check(&next) {
                count[pos] += 1;
                stack.push((next, values.clone()));
            }
        } else {
            if pos >= biggest_pos {
                let (v, _) = stack.pop().unwrap();
                biggest = v;
                biggest_pos = pos;
            } else {
                stack.pop();
            }
        }

        pos = stack.len().wrapping_sub(1);
    }
    print!("{}", count[0]);

    for &i in &count[1..] {
        if i == 0 {
            break;
        }
        print!(", {}", i);
    }
    println!(
        "\nThe biggest truncatable prime in base {} with {} digits is {} == {}\n",
        base,
        offsets.len() - 1,
        radix(&biggest, base),
        radix(&biggest, 10)
    );
}

fn main() {
    let aks = false;
    let check = if aks { aks::aks } else { is_prime::is_prime };

    use std::time::Instant;
    let now = Instant::now();
    for base in 3..10 {
        truncatable(base, check);
    }
    println!("{:?}", now.elapsed());
}
