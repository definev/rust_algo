pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..(n as f64).sqrt() as usize + 1 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    #[test]
    fn is_prime_test() {
        assert_eq!(super::is_prime(1), false);
        assert_eq!(super::is_prime(2), true);
        assert_eq!(super::is_prime(25), false);
    }
}
