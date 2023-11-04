pub fn number_of_steps(mut num: i32) -> i32 {
    let mut steps = 0;
    while num > 0 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num -= 1;
        }
        steps += 1;
    }
    return steps;
}

pub fn number_of_steps_bitwise(mut num: i32) -> i32 {
    let mut steps = 0;
    while num > 0 {
        if (num & 1) == 0 {
            num >>= 1;
        } else {
            num -= 1;
        }
        steps += 1;
    }
    return steps;
}

pub fn number_of_steps_rustish(mut num: i32) -> i32 {
    let mut steps = 0;
    while num > 0 {
        match num % 2 {
            0 => num /= 2,
            _ => num -= 1,
        } 
        steps += 1;
    }
    return steps;
}
