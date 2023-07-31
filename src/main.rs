use crate::solutions::s0014_longest_common_prefix::longest_common_prefix;
mod solutions;

fn main() {
    let rv = longest_common_prefix(vec![String::from("abc"), String::from("abf"), String::from("abbb")]);
    println!("Result {:?}", rv)
}