/// The biggest observation here is that roman numerals seem to all repeat in the same pattern,
/// regardless of what magnitude we're dealing with. This means both the following are congruent:
/// - I, II, III, IV, V, VI, VII, VIII, IX
/// - X, XX, XXX, XL, L, LX, LXX, LXXX, XC
///
/// There's a 'small' symbol, a 'mid-point', and the eventual 'big' symbol. In the first example,
/// the symbols are: I, V, X, but in the second example they are: X, L, C.
///
/// Given that the problem is constrained to inputs below 4000, we only have a few magnitudes to
/// hardcode: (I,V,X), (X,L,C), (X,D,M).
///
/// Then we can loop through our number, picking each digit, converting it to small-mid-big notation
/// using the appropriate magnitude.
///
/// Complexity:
///     Time: O(1)
///         Going through the digits of a 32-bit positive integer less than 4000.
///     Space: O(1)
///         No scaling space is required to compute the outcome.
#[allow(dead_code)]
pub fn int_to_roman(mut num: i32) -> String {
    fn find_correct_10(num: i32) -> (char, char, char) {
        if num == 0 { return ('I', 'V', 'X') }
        if num == 1 { return ('X', 'L', 'C') }
        if num == 2 { return ('C', 'D', 'M') }
        return ('M', 'M', 'M')  // half-baked row, due to problem constraint <= 3999
    }

    fn scale_within_10(small: char, mid: char, big: char, num: i32) -> String {
        if num == 0 { return "".into() }
        if num == 1 { return format!("{}", small) }
        if num == 2 { return format!("{}{}", small, small) }
        if num == 3 { return format!("{}{}{}", small, small, small) }
        if num == 4 { return format!("{}{}", small, mid) }
        if num == 5 { return format!("{}", mid) }
        if num == 6 { return format!("{}{}", mid, small) }
        if num == 7 { return format!("{}{}{}", mid, small, small) }
        if num == 8 { return format!("{}{}{}{}", mid, small, small, small) }
        format!("{}{}", small, big)
    }
    let mut rv: String = "".into();
    let mut power: i32 = 0;
    while num > 0 {
        let (small, mid, big) = find_correct_10(power);
        let mut this_numeral = scale_within_10(small, mid, big, num % 10);
        this_numeral.push_str(&rv);
        rv = this_numeral;

        num = num / 10;
        power += 1;
    }

    rv
}