// 11. Container With Most Water
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_max = 0;
        let mut right_max = 0;
        let mut l = 0;
        let len = height.len();
        let mut r = len - 1;
        let mut res = 0;

        while l < r {
            left_max = i32::max(left_max, height[l]);
            right_max = i32::max(right_max, height[r]);
            let cur_area = (r - l) * (i32::min(left_max, right_max) as usize);
            res = i32::max(res, cur_area as i32);
            if height[l] <= height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        res
    }
}