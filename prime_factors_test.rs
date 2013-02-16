extern mod std;
extern mod primes;

#[cfg(test)]
mod tests {
    #[test]
    fn return_empty_collection_for_one() {
        assert [] == primes::prime_factors(1);
    }

    #[test]
    fn return_list_with_two_for_two() {
        assert [2] == primes::prime_factors(2);
    }

    #[test]
    fn return_list_with_three_for_three() {
        assert [3] == primes::prime_factors(3);
    }

    #[test]
    fn return_list_with_2_2_for_four() {
        assert [2, 2] == primes::prime_factors(4);
    }

    #[test]
    fn return_list_with_2_3_for_six() {
        assert [2, 3] == primes::prime_factors(6);
    }

    #[test]
    fn return_list_with_2_2_2_for_eight() {
        assert [2, 2, 2] == primes::prime_factors(8);
    }

    #[test]
    fn return_list_with_3_3_for_nine() {
        assert [3, 3] == primes::prime_factors(9);
    }
}
