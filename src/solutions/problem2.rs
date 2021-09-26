pub fn solve() -> u64 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 2;

    while b < 4000000 {
        if b % 2 == 0 {
            sum += b;
        }

        let c = a;
        a = b;
        b = c + b;
    }

    return sum;
}