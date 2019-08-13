#![allow(unused)]

use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{cast::ToPrimitive, One, Pow, Zero};

use std::ops::{Index, Mul, Range, Rem, RemAssign};

#[derive(Eq, PartialEq, Clone)]
pub struct Poly {
    data: Vec<BigUint>,
}

lazy_static::lazy_static! {
    static ref ZERO: BigUint = Zero::zero();
}

impl std::fmt::Debug for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = "".to_owned();
        for (i, v) in self.data.iter().enumerate() {
            if !v.is_zero() {
                if !v.is_one() || i == 0 {
                    string.push_str(&v.to_string());
                }
                match i {
                    0 => {}
                    1 => string.push('x'),
                    x => string.push_str(&format!("x^{}", x)),
                }
                string.push_str(" + ");
            }
        }
        if self.len() != 0 {
            string.pop();
            string.pop();
            string.pop();
        }
        f.write_fmt(format_args!("Poly {{ {} }}", string))
    }
}

impl Poly {
    /// creates a new polynomial in the form a + x
    pub fn new(a: BigUint) -> Self {
        //println!("current a: {}", a);
        Self {
            data: vec![a, One::one()],
        }
    }

    /// calculates (self ^ n % (x^r-1))
    pub fn modpow(&mut self, n: &BigUint, r: usize) {
        let mut exponent = n.clone();
        let mut number = Poly {
            data: vec![One::one()],
        };
        // println!("exp: {}\nnumber: {:?}\nself: {:?}\n", exponent, number, self);
        while !exponent.is_zero() {
            if !exponent.is_even() {
                number = &number * &*self;
                while number.len() > r {
                    let new = (number.len() - 1) % r;
                    let x = number.data.pop().unwrap();
                    number.data[new] += x;
                }
            }
            *self = &*self * &*self;
            while self.len() > r {
                let new = (self.len() - 1) % r;
                let x = self.data.pop().unwrap();
                self.data[new] += x;
            }
            exponent >>= 1;
            // println!("exp: {}\nnumber: {:?}\nself: {:?}\n", exponent, number, self);
        }

        self.normalize();
    }

    /// calculates `(self % (x^r - 1)) % n`
    pub fn polynomial_rem_mod(&mut self, r: &BigUint, n: &BigUint) {
        let mut chunks = self.data.chunks_mut(r.to_usize().unwrap());
        let mut low: Vec<BigUint> = chunks.next().unwrap().to_vec();
        for chunk in chunks {
            low.iter_mut().zip(chunk).for_each(|(lo, hi)| {
                *lo += &*hi;
                *lo %= n
            });
        }
        self.data = low;
        *self %= n;
        self.normalize();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// removes truncates all `O` at the end of the polynomial
    pub fn normalize(&mut self) {
        while let Some(n) = self.data.pop() {
            if !n.is_zero() {
                self.data.push(n);
                break;
            }
        }
    }
}

impl Mul for &Poly {
    type Output = Poly;

    fn mul(self, other: &Poly) -> Poly {
        if self.len() == 0 {
            return self.clone();
        }

        let mut res: Vec<BigUint> = std::iter::repeat(Zero::zero())
            .take(self.len() + other.len() - 1)
            .collect();
        for (i, x) in self.data.iter().enumerate() {
            for (j, y) in other.data.iter().enumerate() {
                res[i + j] += x * y;
            }
        }

        Poly { data: res }
    }
}

impl Rem<&BigUint> for Poly {
    type Output = Poly;

    fn rem(mut self, div: &BigUint) -> Self {
        self %= div;
        self
    }
}

impl RemAssign<&BigUint> for Poly {
    fn rem_assign(&mut self, div: &BigUint) {
        self.data.iter_mut().for_each(|v| *v %= div);
    }
}

impl Index<usize> for Poly {
    type Output = BigUint;

    /// returns the coefficient of x^n
    fn index(&self, n: usize) -> &BigUint {
        self.data.get(n).unwrap_or(&ZERO)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn mul() {
        let a = Poly {
            data: vec![5u8.into(), 3u8.into(), 7u8.into()],
        };
        let b = Poly {
            data: vec![11u8.into(), 5u8.into()],
        };
        let x = Poly {
            data: vec![55u8.into(), 58u8.into(), 92u8.into(), 35u8.into()],
        };
        assert_eq!(&a * &b, x);
    }

}
