use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{
    Zero,
    One,
    Pow
};

// this function is slow as heck
fn euler_phi(n: &BigUint) -> BigUint {
    let mut total = BigUint::from(0u8);
    let mut i = BigUint::from(1u8);
    while &i < n {
        if i.gcd(n).is_one() {
            total += 1u8;
        }

        i += 1u8;
    }
    total
}

pub fn aks(n: &BigUint) -> bool {
    // if n = a ^ b for integers a > 1 and b > 1, return false
    for b in 2..=(n.bits() as u32) {
        let a = n.nth_root(b);
        if &a.pow(b) == n {
            //println!("1");
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
        r
    };

    // if 1 < gcd(a, n) < n for some a <= r, return false
    {
        let mut a = BigUint::from(1u8);
        while a < r {
            a += 1u8;

            let gcd = n.gcd(&a);
            if !gcd.is_one() && &gcd < n {
                return false;
            }
        }
    }

    // if n <= r, return true
    if n <= &r {
        return true;
    }

    // for a = 1 to floor(sqrt(euler(r)*log2(n))) do: if (X + a)^n != X^n + a (mod X^r - 1, n), return false
    let max = euler_phi(&r).sqrt()*n.bits();
    // println!("N: {}, max: {}, r: {}", n, max, r);
    let mut a = BigUint::from(1u8);
    while &a < &max {
        //let mut poly = crate::poly::Poly::new(&a, n);
        //println!("poly: {:?}", poly);
        //assert_eq!((n % &r).to_usize().unwrap() + 1, poly.len());
        //assert_eq!(crate::poly::Poly::poly_rem_mod_create(&a, n, &r), poly, "n: {}, a: {}, r: {}", n, a, r);
        //if &a == &BigUint::from(2u8) {
        //    println!("n: {}, r: {}, n % r: {}, len: {}, rem:  {:?}\n", n,r, n %&r, poly.len(), poly);
        //}
        if crate::poly::Poly::poly_rem_mod_create(&a, n, &r) != crate::poly::Poly::what_am_i_even_doing(n, &a, &r) { return false };
        a += 1u8;
    }
    
    // return true
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
        assert_eq!(euler_phi(&BigUint::from(12u8)), BigUint::from(4u8));
        assert_eq!(euler_phi(&BigUint::from(133u8)), BigUint::from(108u8));
    }
}