// Expected to have no multiplication or division.
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = Vec::with_capacity(upper_bound as usize + 1 >> 8);
    let mut is_prime = vec![true; upper_bound as usize + 1];
    for n in 2..=upper_bound {
        if is_prime[n as usize] {
            primes.push(n);
        }

        for m in (n..=upper_bound).step_by(n as usize) {
            is_prime[m as usize] = false;
        }
    }

    primes
}
