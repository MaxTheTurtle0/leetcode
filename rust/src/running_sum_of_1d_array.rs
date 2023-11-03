pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    return nums
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            return nums.as_slice()[0..=idx].iter().sum();
        })
        .collect();
}

pub fn running_sum_smart(nums: Vec<i32>) -> Vec<i32> {
    let mut curr_sum = 0; 
    return nums
        .iter()
        .map(|v| {
            curr_sum += v;
            curr_sum
        })
        .collect();
}

pub fn running_sum_smart_2(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] += nums[i-1];
    }
    return nums
}
