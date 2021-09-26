use crate::algo::strings::num_is_palindrome;

pub fn solve() -> usize {
    let mut largest = 0;

    for i in 100..1000 {
        for j in i..1000 {
            let k = i * j;
            if num_is_palindrome(k) && k > largest {
                largest = k;
            }
        }
    }

    return largest;
}