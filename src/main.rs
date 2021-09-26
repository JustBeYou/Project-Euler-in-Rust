mod solutions;
mod algo;

use solutions::problem1;
use solutions::problem2;
use solutions::problem3;
use solutions::problem4;

fn main() {
    println!("{}", problem1::solve(1, 1000));
    println!("{}", problem2::solve());
    println!("{}", problem3::solve(600851475143));
    println!("{}", problem4::solve());
}
