pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut nums_new = Vec::new();
    for num in nums {
        if nums_new.contains(&num) == true {
            nums_new.retain(|&x| x != num);
        } else {
            nums_new.push(num);
        }
    } 

    return nums_new[0];
}


pub fn single_number_bitwise(nums: Vec<i32>) -> i32 {
    let mut answer = 0;
    for num in nums {
        answer ^= num;
    }
    return answer;
}

pub fn single_number_bitwise_rustish(nums: Vec<i32>) -> i32 {
    return nums.iter().fold(0, |acc, &num| acc ^ num);
}
