struct Solution {}

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::Deref;
use std::u32;

// 697. Degree of an Array
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        nums.clone().into_iter()
            .for_each(|num| {
                let mut count = map.entry(num).or_insert(0);
                *count += 1;
            });
        let max_freq = *(map.values().max().unwrap());
        let mut set = map.into_iter().filter(|&m| m.1 == max_freq)
            .map(|x| x.0)
            .collect::<HashSet<_>>();

        set.into_iter()
            .map(|elem| {
                let first = nums.iter().position(|i| *i == elem).unwrap();
                let last = nums.iter().rposition(|i| *i == elem).unwrap();
                last - first + 1
            })
            .min()
            .unwrap() as i32
    }
}

fn main() {
    let char_vec = vec!['A', 'B'];
    let int_vec = vec![1, 2, 2, 3, 1, 4, 2];

    let n = 19923;
    println!("{:?}", Solution::find_shortest_sub_array(int_vec));
}
