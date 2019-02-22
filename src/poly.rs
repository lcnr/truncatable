#![allow(unused)]

use num_bigint::BigUint;
use num_traits::{
    Pow,
    cast::ToPrimitive,
    Zero,
    One
};

#[derive(Eq, PartialEq)]
pub struct Poly {
    data: Vec<BigUint>,
}

impl std::fmt::Debug for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter().map(|a| format!("{}", a))).finish()
    }
}


fn count(n: &BigUint, c: &BigUint) -> BigUint {
    n.factorial()/(c.factorial() * (n - c).factorial())
}

impl Poly {
    /// creates a new polynomial in the form (x + a)^n
    pub fn new(a: &BigUint, n: &BigUint) -> Self {
        let mut poly = Self {
            data: vec![],
        };
        let mut i = BigUint::from(0u8);
        while &i <= n {
            poly.data.push(count(n, &i) * a.pow(&n.to_u32().unwrap() - i.to_u32().unwrap()));
            i += 1u8;
        }
        poly
    }

    /// `(x^n+a)%(x^r -1)`
    pub fn what_am_i_even_doing(n: &BigUint, a: &BigUint, r: &BigUint) -> Self {
        let pos = (n % r).to_usize().unwrap();
        let mut data = vec![a.clone()];
        if pos > 0 {
            data = data.into_iter().chain((1..=pos).map(|p| if p < pos { Zero::zero() } else { One::one() })).collect();
        }

        Self {
            data
        }
    }

    /// calculates `((x + a)^n % (x^r - 1)) % n`
    pub fn poly_rem_mod_create(a: &BigUint, n: &BigUint, r: &BigUint) -> Self {
        Self::what_am_i_even_doing(n, &a.modpow(n, n), r)
    }

    /// calculates `(self % (x^r - 1)) % n`
    pub fn polynomial_rem_mod(&mut self, r: &BigUint, n: &BigUint) {
        let mut chunks = self.data.chunks_mut(r.to_usize().unwrap());
        let mut low: Vec<BigUint> = chunks.next().unwrap().to_vec();
        for chunk in chunks {
            low.iter_mut().zip(chunk).for_each(|(lo, hi)| { *lo += &*hi; *lo %= n });
        }
        low.iter_mut().for_each(|lo| *lo %= n);
        while let Some(n) = low.pop() {
            if !n.is_zero() {
                low.push(n);
                break;
            }
        }
        self.data = low;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn simple() {
        let poly = Poly::new(&BigUint::from(5u8), &BigUint::from(3u8));
        assert_eq!(&poly.data, &[BigUint::from(125u8), BigUint::from(75u8), BigUint::from(15u8), BigUint::from(1u8)]);
    }
}