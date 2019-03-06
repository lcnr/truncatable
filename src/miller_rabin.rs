use num_bigint::{ BigUint, RandBigInt };
use num_integer::Integer;

use rand::thread_rng;


fn get_rd(n: &BigUint) -> (usize, BigUint) {
    let mut odd: BigUint = n - 1u8;
    let mut sup = 0;
    while odd.is_even() {
        sup += 1;
        odd >>= 1;
    }

    (sup, odd)
}

pub fn miller_rabin(num: &BigUint, k: u32) -> bool {
    let (r, d) = get_rd(num);
    let upper_limit = num - 1u8;
    let one = BigUint::from(1u8);
    let two = BigUint::from(2u8);

    'witness: for _ in 0..k {
        let a = thread_rng().gen_biguint_range(&two, &upper_limit);
        let mut x = a.modpow(&d, num);
        if &x == &one || &x == &upper_limit {
            continue 'witness;
        }
        
        for _ in 0..r {
            x = x.modpow(&two, num);
            if &x == &one || &x == &upper_limit {
                continue 'witness;
            }
        }

        return false;
    }
    true
}