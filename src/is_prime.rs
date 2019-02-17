use num_traits::Zero;
use num_traits::Pow;
use num_traits::ToPrimitive;
use num_integer::Integer;
use num_bigint::BigUint;

use crate::miller_rabin::miller_rabin;


// only works for num > 3
pub fn is_prime(num: &BigUint) -> bool {
    if let Some(num) = num.to_u64() {
        primal::is_prime(num)
    }
    else {
        !(num.is_even() || num.quick_rem(3) == 0|| !miller_rabin(num, 32))
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