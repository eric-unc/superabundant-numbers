use counter::Counter;
use prime_factorization::Factorization;

const MAX: u128 = 1_000_000_000_000_000_000_000_000_000_000;

fn lhs(n: u128, factorization: &Counter<u128>) -> f64 {
    let mut sum_of_divisors = 1;
    for (p, a) in factorization {
        let a = *a as u32;
        sum_of_divisors *= (p.pow(a + 1) - 1) / (*p - 1);
    }
    sum_of_divisors as f64 / n as f64
}

fn main() {
    let mut increment: u128 = 1;
    let mut sum: u128 = 1;
    let mut max_ratio: f64 = 1.0;
    let mut max_prime: u128 = 1;
    let mut prime_factors = Counter::new();

    while sum <= MAX {
        sum += increment;
        let temp_prime_factors = prime_factors.clone();

        let factorization = Factorization::run(sum / increment);
        let mut factorization: Counter<_> = factorization.factors.into_iter().collect();
        factorization.extend(&temp_prime_factors);

        let ratio = lhs(sum, &factorization);
        if ratio > max_ratio {
            max_ratio = ratio;

            println!("{sum}");

            let greatest_prime_factor = factorization.keys().max().unwrap();

            if *greatest_prime_factor > max_prime {
                max_prime = *greatest_prime_factor;
                increment *= max_prime;
                prime_factors[&max_prime] = 1;
            };
        }
    }
}