struct Solution {}
use std::collections::{HashMap, HashSet, VecDeque};
// 560. Subarray Sum Equals K
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut map = HashMap::new();
        map.insert(0, 1);

        nums.into_iter()
            .for_each(|num| {
                sum += num;
                if let Some(frq) = map.get_mut(&(sum - k)) {
                    count += (*frq);
                }
                let mut frq = map.entry(sum).or_insert(0);
                (*frq) += 1;
            });

        count
    }
}