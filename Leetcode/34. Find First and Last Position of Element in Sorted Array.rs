// 34. Find First and Last Position of Element in Sorted Array
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![nums.iter().position(|num| num == &target),
             nums.iter().rposition(|num| num == &target)]
            .into_iter()
            .map(|res| {
                if let Some(pos) = res {
                    pos as i32
                } else {
                    -1
                }
            })
            .collect::<Vec<_>>()
    }
}