use num_traits::Zero;
use num_traits::Pow;
use num_bigint::BigUint;

pub struct PrimeBook(Vec<BigUint>);

impl PrimeBook {
    pub fn new() -> Self {
        PrimeBook(vec![BigUint::from(2u8), BigUint::from(3u8), BigUint::from(5u8), BigUint::from(7u8)])
    }

    pub fn is_prime(&mut self, num: &BigUint) -> bool {
        if let Ok(_) = self.0.binary_search(&num) {
            true
        }
        else {
            let sqrt = num.sqrt();
            match self.0.binary_search(&sqrt) {
                Err(index) => if index == self.0.len() {
                    let mut last = self.0.last().unwrap().clone();
                    while &last <= &sqrt {
                        last += 2u8;
                        if self.is_prime(&last) {
                            self.0.push(last.clone());
                        }
                    }
                    self.0.iter().all(|item| num % item != Zero::zero())
                }
                else {
                    self.0.iter().take(index).all(|item| num % item != Zero::zero())
                },
                Ok(index) => if &self.0.get(index).unwrap().pow(2u8) == num {
                    false
                }
                else {
                    self.0.iter().take(index + 1).all(|item| num % item != Zero::zero())
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
