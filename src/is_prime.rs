use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::Pow;
use num_traits::ToPrimitive;
use num_traits::Zero;

use crate::miller_rabin::miller_rabin;

// only works for num > 3
pub fn is_prime(num: &BigUint) -> bool {
    let bits = num.bits() as u32;
    if bits <= 8 {
        match num.to_u8().expect("to_u8") {
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31 | 37 | 41 | 43 | 47 | 53 | 59 | 61
            | 67 | 71 | 73 | 79 | 83 | 89 | 97 | 101 | 103 | 107 | 109 | 113 | 127 | 131 | 137
            | 139 | 149 | 151 | 157 | 163 | 167 | 173 | 179 | 181 | 191 | 193 | 197 | 199 | 211
            | 223 | 227 | 229 | 233 | 239 | 241 | 251 => true,
            _ => false,
        }
    } else if num.is_even() || !miller_rabin(num, 16) || (num % 3u8).is_zero() {
        false
    } else {
        let sqrt = num.sqrt();
        if &sqrt.pow(2u8) == num {
            false
        } else if let Some(sqrt) = sqrt.to_u32() {
            let mut i = 5;
            while i < sqrt {
                if (num % i).is_zero() || (num % (i + 2)).is_zero() {
                    return false;
                }
                i += 6;
            }
            true
        } else {
            let mut i = BigUint::from(5u8);
            while &i < &sqrt {
                if (num % &i).is_zero() {
                    return false;
                }
                i += 2u8;
                if (num % &i).is_zero() {
                    return false;
                }
                i += 4u8;
            }
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::is_prime;
    use num_bigint::BigUint;

    #[test]
    fn two_is_prime() {
        let two = BigUint::from(2u8);
        assert!(is_prime(&two));
    }

    #[test]
    fn thirty_seven_is_prime() {
        let thirty_seven = BigUint::from(37u8);
        assert!(is_prime(&thirty_seven));
    }
}
