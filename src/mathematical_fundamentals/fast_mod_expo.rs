pub fn fast_mod_expo_recursive(a: u64, n: u64, modulo: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    if n % 2 == 0 {
        // a^n % modulo == ((a^2) % modulo)^(n/2) % module
        return fast_mod_expo_recursive(a.pow(2) % modulo, n / 2, modulo);
    } else {
        // a^n % modulo == a * (((a^2) % modulo)^(n/2) % module)
        return (a * fast_mod_expo_recursive(a, n - 1, modulo)) % modulo;
    }
}

struct FMEIterative(u64, u64, u64);

pub fn fast_mod_expo_iterative(a: u64, n: u64, modulo: u64) -> u64 {
    let mut iter = FMEIterative(a, n, 1);

    while iter.1 > 0 {
        if iter.1 % 2 == 0 {
            iter.0 = iter.0.pow(2) % modulo;
            iter.1 = iter.1 / 2;
        } else {
            iter.1 -= 1;
            iter.2 = (iter.2 * iter.0) % modulo;
        }
    }

    return iter.2;
}

#[cfg(test)]
mod test {
    #[test]
    fn fast_mod_expo_recursive_test() {
        assert_eq!(super::fast_mod_expo_recursive(2, 4, 9), 7);
        assert_eq!(super::fast_mod_expo_recursive(2, 5, 9), 5);
        assert_eq!(super::fast_mod_expo_recursive(2, 100, 9), 7);
    }

    #[test]
    fn fast_mod_expo_iterative_test() {
        assert_eq!(super::fast_mod_expo_iterative(2, 4, 9), 7);
        assert_eq!(super::fast_mod_expo_iterative(2, 5, 9), 5);
        assert_eq!(super::fast_mod_expo_iterative(2, 100, 9), 7);
    }
}
