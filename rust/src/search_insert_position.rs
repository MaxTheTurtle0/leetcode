pub fn search_insert_position(nums: Vec<i32>, target: i32) -> i32 {
    for idx in 0..nums.len() {
        if nums[idx] >= target {
            return idx as i32;
        }
    }
    return nums.len() as i32;
}
