test: prime_factors_test.rs primes.rs
	rustc --lib primes.rs	
	rustc prime_factors_test.rs -L . --test -o primes_test

clean:
	rm -rf *.dylib *.dSYM primes_test
