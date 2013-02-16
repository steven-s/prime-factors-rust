extern mod std;
extern mod primes;

#[cfg(test)]
mod tests {
    #[test]
    fn one() {
        assert [] == primes::prime_factors(1);
    }

    #[test]
    fn two() {
        assert [2] == primes::prime_factors(2);
    }

    #[test]
    fn three() {
        assert [3] == primes::prime_factors(3);
    }

    #[test]
    fn four() {
        assert [2, 2] == primes::prime_factors(4);
    }

    #[test]
    fn six() {
        assert [2, 3] == primes::prime_factors(6);
    }

    #[test]
    fn eight() {
        assert [2, 2, 2] == primes::prime_factors(8);
    }

    #[test]
    fn nine() {
        assert [3, 3] == primes::prime_factors(9);
    }
}
