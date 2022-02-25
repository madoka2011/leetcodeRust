// 42. Trapping Rain Water
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut len = height.len();
        let mut r = len - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut res = 0;
        while l < r {
            left_max = i32::max(left_max, height[l]);
            right_max = i32::max(right_max, height[r]);
            if height[l] <= height[r] {
                let mut diff = left_max - height[l];
                if diff >= 0 {
                    res += diff;
                }
                l += 1;
            } else {
                let mut diff = right_max - height[r];
                if diff >= 0 {
                    res += diff;
                }
                r -= 1;
            }
        }

        res
    }
}