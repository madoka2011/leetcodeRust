// 1013. Partition Array Into Three Parts With Equal Sum
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let mut sum = 0;
        let mut count = 0;

        let mut target = arr.iter().sum::<i32>();
        if target % 3 != 0 {
            return false;
        }
        target = target / 3;
        let mut l = 50001;
        let mut r = 0;
        println!("{}", arr.len());
        println!("{}", target);
        for (index, num) in arr.clone().into_iter().enumerate() {
            sum += num;
            if sum == target {
                println!("l{}", index);
                l = index;
                break;
            }
        }

        sum = 0;
        for (index, num) in arr.clone().into_iter().enumerate().rev() {
            sum += num;
            if sum == target {
                println!("r{}", index);
                r = index;
                break;
            }
        }


        l + 1 < r
    }
}