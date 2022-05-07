pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = vec![];
    let mut dividend = n;
    let mut divisor = 2;

    while dividend > 1 {
        while dividend % divisor == 0 {
            primes.push(divisor);
            dividend /= divisor;
        }
        divisor += 1;
    }
    primes
}
