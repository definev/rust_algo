use std::fmt::Display;

pub struct Factorize {
    primes: Vec<usize>,
    expos: Vec<i32>,
}

impl Display for Factorize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();

        for (index, prime) in self.primes.iter().enumerate() {
            let mut expr = String::from(prime.to_string());

            if self.expos[index] > 1 {
                expr.push_str(format!("^{}", self.expos[index]).as_str());
            }
            if index < self.primes.len() - 1 {
                expr.push_str(" * ");
            }

            buf.push_str(expr.as_str());
        }

        write!(f, "{buf}")
    }
}

pub fn factorize_prime(n: usize) -> Factorize {
    let mut primes = Vec::new();
    let mut expos = Vec::new();
    let mut prime = 2;
    let mut n = n;

    while n > 1 {
        if n % prime == 0 {
            if primes.is_empty() {
                primes.push(prime);
                expos.push(1);
                n /= prime;
                continue;
            }
            if primes.last().unwrap() != &prime {
                primes.push(prime);
                expos.push(1);
            } else {
                if let Some(expo) = expos.last_mut() {
                    *expo += 1;
                }
            }
            n /= prime;
        } else {
            prime += 1;
        }
    }

    return Factorize { primes, expos };
}

#[cfg(test)]
mod test {
    #[test]
    fn factorize_prime_test() {
        assert_eq!(super::factorize_prime(1).to_string(), "");
        assert_eq!(super::factorize_prime(6).to_string(), "2 * 3");
        assert_eq!(super::factorize_prime(26).to_string(), "2 * 13");
        assert_eq!(super::factorize_prime(27).to_string(), "3^3");
        assert_eq!(super::factorize_prime(100).to_string(), "2^2 * 5^2");
        assert_eq!(super::factorize_prime(1412).to_string(), "2^2 * 353");
    }
}
