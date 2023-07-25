/// The intuition of this solution is that as we zig-zag up and down, each row of the pattern
/// is determined using some combination of: num_rows and current_row.
///
/// For example, observe that indexes 0, num_rows*2, num_rows*4, num_rows*6, etc. populate the
/// first row. This makes sense because as we zig down, we travel num_rows, then as we zag up,
/// we travel num_rows again.
///
/// Using these top-row markers, the second row can be expressed as +/- 1 from the top row.
/// Similarly, the third row can be expressed as +/- 2 from the top row.
///
/// As such, the algorithm traverses row by row, and in an inner loop, column by column
/// It first computes the index at the top of the first column using (num_rows-1) x 2 x column.
/// Then it calculates the left and right indices, which is +/- the current_row.
/// It throws away any value that is out of bounds.
/// Then it adds left & right to our output string.
///
/// Ultimately it returns the output string.
///
/// Complexity:
///     Time: O(n)
///         We touch all elements of the input string a constant number of times.
///     Space: O(1)
///         Not counting the returned output, we don't store anything that scales with input size.
#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 { return s }

    let bytes: &[u8] = s.as_bytes();
    let length: usize = bytes.len();
    let mut rv: Vec<u8> = Vec::new();

    for current_row in 0..num_rows.min(length as i32) {
        for column in 0.. {
            let core = ((num_rows-1) * 2) * column;

            let right = core + current_row;
            let left = core - current_row;

            if left >= length as i32 { break; }
            if left >= 0 && current_row != num_rows - 1 { rv.push(bytes[left as usize]); }

            if right == left { continue }
            if right >= length as i32 { break }
            rv.push(bytes[right as usize]);
        }
    }
    String::from_utf8(rv).unwrap()
}