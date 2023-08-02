/// Use a similar traversal policy as the previous problem (3sum), where we keep three pointer. But
/// instead of looking for 0, we measure the distance from our target, and any time we find a sum
/// with smaller distance than we've previously seen, we mark the sum and the distance.
///
/// Complexity:
///     Time: O(n^2)
///         For every element, touch n-order of other elements
///     Space: O(1)
///         No additional space requirement to produce the output
#[allow(dead_code)]
pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();

    let mut closest: i32 = 100_001; // problem constraint -1k to 1k
    let mut closest_dist: i32 = 100_001;

    let mut last_left = *nums.get(0).unwrap() - 1;

    for idx_left in 0..nums.len() {
        let left = *nums.get(idx_left).unwrap();
        if left == last_left { continue; }
        last_left = left;

        let mut idx_mid = idx_left + 1;
        let mut idx_right = nums.len() - 1;

        while idx_mid < idx_right {
            let mid = *nums.get(idx_mid).unwrap();
            let right = *nums.get(idx_right).unwrap();

            let sum = left + mid + right;
            let sum_dist = (target - sum).abs();

            if sum == target { return sum; }

            if sum_dist < closest_dist {
                println!("Found: sum={sum}, sum_dist={sum_dist}");
                closest = sum;
                closest_dist = sum_dist;
            }
            if sum < target { idx_mid += 1; }
            else if sum > target { idx_right -= 1; }
        }
    }

    closest
}
