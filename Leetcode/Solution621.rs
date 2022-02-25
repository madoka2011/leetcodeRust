// 621. Task Scheduler
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut frqs = vec![0; 26];
        tasks.iter()
            .for_each(|c| {
                frqs[*c as usize - 'A' as usize] += 1;
            });

        frqs.sort_by(|x1, x2| x1.cmp(x2));

        let f_max = frqs[25];
        let mut idle_time = (f_max - 1) * n;

        for i in (0..=24).rev() {
            idle_time -= i32::min(f_max - 1, frqs[i as usize]);
            if idle_time <= 0 {
                break;
            }
        }
        idle_time = i32::max(0, idle_time);

        idle_time + tasks.len() as i32
    }
}