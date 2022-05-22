mod sieve {
    use std::collections::HashMap;

    type Composite = u64;
    type Prime = u64;
    type Number = u64;

    pub struct Primes {
        curr: Number,
        sieve: HashMap<Composite, Prime>,
    }

    impl Primes {
        pub fn primes() -> Primes {
            Primes {
                curr: 2,
                sieve: HashMap::new(),
            }
        }

        fn is_prime(&mut self, n: Number) -> bool {
            if let Some(prime) = self.sieve.remove(&n) {
                self.mark_composite(n / prime + 1, prime);
                false
            } else {
                self.mark_composite(2, n);
                true
            }
        }

        fn mark_composite(&mut self, times: Number, prime: Prime) {
            (times..)
                .find(|i| !self.sieve.contains_key(&(i * prime)))
                .and_then(|i| self.sieve.insert(i * prime, prime));
        }
    }

    impl Iterator for Primes {
        type Item = Prime;

        fn next(&mut self) -> Option<Self::Item> {
            let prime = (self.curr..).find(|&n| self.is_prime(n))?;
            self.curr = prime + 1;
            Some(prime)
        }
    }
}

use crate::sieve::Primes;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    Primes::primes()
        .take_while(|&prime| prime <= upper_bound)
        .collect()
}
