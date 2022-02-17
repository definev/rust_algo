pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let x_str = x.to_string();

    if x_str == x_str.chars().rev().collect::<String>() {
        return true;
    }
    return false;
}
