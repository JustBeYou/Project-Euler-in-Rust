pub struct Sieve<const N: usize> {
    is_prime: Vec<bool>
}

impl<const N: usize> Sieve<N> {
    pub fn new() -> Self {
        let mut is_prime = vec![true; N];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..N {
            if is_prime[i] {
                for j in (i * i..N).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        Self { is_prime }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        return self.is_prime[n];
    }
}