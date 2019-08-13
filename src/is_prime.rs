use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::ToPrimitive;

use crate::miller_rabin::miller_rabin;

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
    } else if num.is_even() || !miller_rabin(num, 64) {
        false
    } else {
        true
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
