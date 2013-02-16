#[link(name = "primes", vers = "1.0")];
pub fn prime_factors(number : int) -> ~[int] {
    // make number mutable; this is fixed in later versions so args can
    // be made mutable
    let mut number = number; 
    let mut prime_numbers = ~[];
    let mut candidate = 2;

    while number > 1 {
        while number % candidate == 0 {
            prime_numbers.push(candidate);
            number /= candidate;
        }
        candidate += 1;
    }

    return prime_numbers;
}
