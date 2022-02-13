pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut marks: Vec<Option<bool>> = Vec::new();
    for _ in 2..(n + 1) {
        marks.push(None);
    }

    'sieving: loop {
        let sieve = marks
            .iter()
            .enumerate()
            .find(|(_, marked)| **marked == None);

        match sieve {
            Some((sieve, _)) => {
                for (index, marked) in marks.iter_mut().enumerate() {
                    if *marked != None {
                        continue;
                    }
                    if index == sieve {
                        *marked = Some(true);
                        continue;
                    }
                    if (index + 2) % (sieve + 2) == 0 {
                        *marked = Some(false);
                    }
                }
            }
            None => break 'sieving,
        }
    }

    let mut res: Vec<usize> = Vec::new();
    for (index, marked) in marks.iter().enumerate() {
        if *marked == Some(true) {
            res.push(index + 2);
        }
    }
    return res;
}

#[cfg(test)]
mod test {
    #[test]
    fn sieve_of_eratosthenes_test() {
        assert_eq!(super::sieve_of_eratosthenes(1), vec![]);
        assert_eq!(super::sieve_of_eratosthenes(5), vec![2, 3, 5]);
        assert_eq!(super::sieve_of_eratosthenes(15), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(
            super::sieve_of_eratosthenes(40),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37],
        );
    }
}
