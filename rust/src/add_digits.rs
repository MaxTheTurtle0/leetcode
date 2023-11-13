pub fn add_digits(mut num: i32) -> i32 {
    while num > 9 {
        num = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .sum::<i32>();
    }
    return num;
}

pub fn add_digits_smart(mut num: i32) -> i32 {
    while num > 9 {
        num = num % 10 + num / 10;
    } 
    return num;
}
