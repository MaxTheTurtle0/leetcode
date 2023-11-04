pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result = Vec::new(); 
    for i in 1..=n {
        let mut string = String::from("");
        if i % 3 == 0 {
            string.push_str("Fizz");
        }
        if i % 5 == 0 {
            string.push_str("Buzz");
        }

        if string.is_empty() {
            string.push_str(i.to_string().as_str());
        }
        result.push(string);
    }
    return result;
}

pub fn fizz_buzz_rustish(n: i32) -> Vec<String> {
    return (1..=n)
        .map(|num| match (num % 3, num % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => num.to_string(),
        })
        .collect();
}
