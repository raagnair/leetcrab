/// The intuition here can be described in a few basic steps:
///     Firstly, we know that regardless of what the max area is, it can not be wider than the width
///         formed by the verticals on the outside extremes of the input array. So maybe we can
///         start our search there, and move towards the center. Then the question becomes, how do
///         we move towards the center?
///     Given any two verticals in our input array, we know that the shorter of our two verticals is
///         the limiting factor in the area calculation. For example, for verticals (5, 10), the 5
///         is the one determining the height of the area.
///     Therefore, as we move from outside to inside in search of the tallest area, it doesn't make
///         sense for us to move the taller vertical. Since the shorter height is the one that sets
///         the height of the resultant area, moving the taller vertical keeps the resultant area's
///         height the same and decreases the width by 1, which always lends a smaller area.
///
/// The three observations above lead us to our simple algorithm. Two pointers starting from the
/// outside of the input array, moving towards the center. At each iteration, record any area that
/// is the biggest so far. Then find whichever index has the shorter vertical and move that index
/// closer to the center for the next iteration.
///
/// Complexity:
///     Time: O(n)
///         One traversal through the input.
///     Space: O(1)
///         No scaling memory required for the algorithm.
#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    let length = height.len();

    let mut greatest_area: i32 = -1;
    let mut left_idx = 0;
    let mut right_idx = length - 1;

    while left_idx < right_idx {
        let left_height = height.get(left_idx).unwrap();
        let right_height = height.get(right_idx).unwrap();
        let area = left_height.min(right_height) * (right_idx - left_idx) as i32;

        if area > greatest_area { greatest_area = area; }

        if left_height < right_height { left_idx += 1; }
        else { right_idx -= 1; }
    }
    greatest_area
}