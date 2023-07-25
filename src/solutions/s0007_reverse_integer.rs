/// A trivial solution to the problem.
///
/// Get the absolute value, read as string, reverse. Try to parse that as an i32.
/// If successful, multiply by -1 if original input was negative, then return.
/// If not successful, return 0.
///
/// Complexity:
///     Time: O(1)
///         The steps of the algo doesn't scale as the input gets larger.
///     Space: O(log(n))
///         Need a string that holds all the digits of the input, which can be expressed
///         vaguely as log_10(n), which is in the order of log(n).
#[allow(dead_code)]
pub fn reverse(x: i32) -> i32 {
    let abs_x: u32 = x.abs() as u32;
    let cardinality = if x < 0 {-1} else {1};
    let flip_str: String= abs_x.to_string().chars().rev().collect();

    match flip_str.parse::<i32>() {
        Err(_) => return 0,
        Ok(flip_num_raw) => {
            let flip_num = flip_num_raw * cardinality;
            return flip_num;
        }
    }
}