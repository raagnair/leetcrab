use crate::solutions::s003_longest_nonrepeat_substring::length_of_longest_substring;
mod solutions;

fn main() {
    let rv = length_of_longest_substring("pwwkew".into());
    println!("Result {:?}", rv)
}