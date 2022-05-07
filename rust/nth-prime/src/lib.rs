use std::collections::HashMap;

type Composite = u32;
type Prime = u32;
type Number = u32;

struct Primes {
    curr: Number,
    sieve: HashMap<Composite, Prime>,
}

impl Primes {
    fn primes() -> Primes {
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
        if let Some(prime) = (self.curr..).find(|&n| self.is_prime(n)) {
            self.curr = prime + 1;
            Some(prime)
        } else {
            None
        }
    }
}

pub fn nth(n: u32) -> u32 {
    let mut primes = Primes::primes();
    primes.nth(n as usize).unwrap()
}
