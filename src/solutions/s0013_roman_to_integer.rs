/// General approach is to traverse from right to left. As we encounter larger symbols, we add to
/// our final number. Any time we encounter a symbol that's smaller than our largest thus far, we
/// subtract it from our running sum.
///
/// I'm using a few if-statements here rather than a Map<char, i32> because I believe at such small
/// scale the if-statements are faster.
///
/// Complexity:
///     Time: O(n)
///         Traverses through the string once
///     Space: O(1)
///         No scaling space requirements to compute the output
#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    fn from_char_to_val(input: char) -> i32 {
        if input == 'I' { return 1 }
        if input == 'V' { return 5 }
        if input == 'X' { return 10 }
        if input == 'L' { return 50 }
        if input == 'C' { return 100 }
        if input == 'D' { return 500 }
        return 1000
    }

    let mut rv = 0;
    let mut highest_val_found = -1; // some number smaller than all possible inputs
    for &curr_byte in s.as_bytes().iter().rev() {
        let curr_val = from_char_to_val(curr_byte as char);
        if curr_val >= highest_val_found {
            highest_val_found = curr_val;
            rv += curr_val;
        } else {
            rv -= curr_val;
        }
    }
    rv
}