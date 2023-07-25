/// We build our return value one character at a time. But first we must inspect the first character
/// to determine if there is a +/- present. We build our return value by folding over our characters
/// and starting with 0, then multiplying our accumulated value by 10 so that we can add the latest
/// character.
///
/// To handle overflows, we guard the two risky operations:
///     - Multiplying our accumulator by 10
///     - Adding our latest digit to our accumulator
/// In order to guard from overflows, we use the saturated_* methods available on i32 which takes
/// magnetizes to the min/max i32 value for free.
///
/// Cmmplexity:
///     Time: O(n)
///         We have to parse through the length of our string.
///     Space: O(1)
///         Not counting the return value, we have no additional data stored for the algorithm.
#[allow(dead_code)]
pub fn my_atoi(s: String) -> i32 {
    let (s, ordinal) = match s.trim_start().strip_prefix('-') {
        Some(rest) => (rest, -1),
        None => (s.trim_start().strip_prefix('+').unwrap_or(s.trim_start()), 1)
    };
    s.chars()
        .map(|c: char| c.to_digit(10))
        .take_while(Option::is_some)
        .map(Option::unwrap)
        .map(|c| c as i32)
        .fold(0i32, |sum, c| sum.saturating_mul(10).saturating_add(c * ordinal as i32))
}