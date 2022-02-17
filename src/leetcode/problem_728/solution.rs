pub fn is_self_dividing_number(number: i32) -> bool {
    let numb_str = number.to_string();

    for digit in numb_str.chars() {
        let digit = digit.to_digit(10).unwrap() as i32;
        if digit == 0 || number % digit != 0 {
            return false;
        }
    }

    return true;
}

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut res = vec![];

    for num in left..right + 1 {
        if is_self_dividing_number(num) {
            res.push(num);
        }
    }

    return res;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_case() {
        assert_eq!(
            super::self_dividing_numbers(1, 20),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15]
        );
    }
}
