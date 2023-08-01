use crate::solutions::s0015_3sum::three_sum;
mod solutions;

fn main() {
    let rv = three_sum(vec![-1,0,1,2,-1,-4]);
    println!("Result {:?}", rv)
}