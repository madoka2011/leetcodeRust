// 670. Maximum Swap
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut res = num.to_string();
        let num = num.to_string();
        let mut back = vec![];
        back.push(num.chars().nth_back(0).unwrap());
        for (index, c) in num.chars().rev().skip(1).enumerate() {
            back.insert(0, u8::max(num.chars().nth(index).unwrap() as u8, num.chars().nth(index + 1).unwrap() as u8) as char);
        }
        println!("{:?}", back);
        1
    }
}