pub fn is_palindrome(s: &str) -> bool {
    let mut chars = s.chars();
    let mut first = chars.next();
    let mut last = chars.next_back();
    
    while first.is_some() && last.is_some() {
        if first.unwrap() != last.unwrap() {
            return false;
        }
        first = chars.next();
        last = chars.next_back();
    }
    return true;
}

pub fn num_is_palindrome(n: usize) -> bool {
    return is_palindrome(&n.to_string());
}