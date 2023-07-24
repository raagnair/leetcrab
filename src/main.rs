use crate::solutions::s0001_two_sums::two_sum;
mod solutions;

fn main() {
    let rv = two_sum(vec![1, 2, 3, 10, 20], 23);
    println!("Result {:?}", rv)
}