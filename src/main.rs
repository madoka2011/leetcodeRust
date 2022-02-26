struct Solution {}

use std::collections::{HashMap, HashSet, VecDeque};
use std::u32;

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

fn main() {
    let char_vec = vec!['A', 'B'];
    let int_vec = vec![5, 7, 7, 8, 8, 10];

    let n = 28;
    println!("{:?}", Solution::search_range(int_vec, n));
}
