use num_bigint::BigUint;
use num_integer::{
    Integer,
    Roots
};
use num_traits::{
    ToPrimitive,
    Zero,
    One,
    Pow
};

use crate::poly::Poly;

// this function is slow as heck
fn euler_phi(n: u32) -> u32 {
    let mut total = Zero::zero();
    for i in 1..n {
        if i.gcd(&n).is_one() {
            total += 1;
        }
    }
    total
}

pub fn aks(n: &BigUint) -> bool {
    // if n = a ^ b for integers a > 1 and b > 1, return false
    for b in 2..=(n.bits() as u32) {
        let a = n.nth_root(b);
        if &a.pow(b) == n {
            return false
        }
    }

    // find the smallest r such that the multiplicative order of n modulo r > (log2 n)^2
    let goal: BigUint = BigUint::from(n.bits()).pow(2u8);
    let r = { 
        let mut r = BigUint::from(1u8);
        'outer: loop {
            r += 1u8;
            let mut k = BigUint::from(0u8);
            while k < goal {
                k += 1u8;
                let res = n.modpow(&k, &r);
                if res.is_zero() || res.is_one() {
                    continue 'outer;
                }
            }
            break 'outer;
        }
        r.to_u32().expect("r does not fit into a usize")
    };
    // if 1 < gcd(a, n) < n for some a <= r, return false
    {
        for a in 2..=r {
            let gcd = n.gcd(&a.into());
            if !gcd.is_one() && &gcd < n {
                return false;
            }
        }
    }

    // if n <= r, return true
    if let Some(n) = n.to_u32() {
        if n <= r {
            return true;
        }
    }

    // for a = 1 to floor(sqrt(euler(r))*log2(n)) do: if (X + a)^n != X^n + a (mod X^r - 1, n), return false
    //
    // note: max is currently larger than floor(sqrt(euler(r)*log2(n))) as n.bits() is used.
    let max: u32 = euler_phi(r).sqrt() * n.bits() as u32;
    for a in 1..max {
        let mut poly = Poly::new(a.into());
        poly.modpow(n, r.to_usize().unwrap());
        poly %= n;
        
        // check if `poly == X^n + a (mod X^r - 1)`
        // `X^n + a mod X^r - 1` is equal to a + x ^ (n % r) == a + x ^ v
        // does not work if `v` is zero, this does not matter as `n % r = 0` means that n is not prime`,
        // which is correctly identified
        let v = (n % r).to_usize().unwrap() + 1;
        if !(poly[0].to_u32().map_or(false, |o| o == a)) && poly[v].is_one() && {
            let mut a = true;
            for _ in 1..v {
                a &= poly[v].is_zero();
            }
            a
        } {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn aks_test() {
        assert!(aks(&BigUint::from(31u8)));
    }

    #[test]
    fn euler_phi_test() {
        assert_eq!(euler_phi(12), 4);
        assert_eq!(euler_phi(133), 108);
    }
}