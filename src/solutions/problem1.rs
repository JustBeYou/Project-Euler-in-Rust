pub fn solve(from: u64, to: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in from..to {
        if i % 3 == 0 {
            sum += i;
        } else if i % 5 == 0 {
            sum += i;
        }
    }
    return sum;
}