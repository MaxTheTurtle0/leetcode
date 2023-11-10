pub fn length_of_last_word(s: String) -> i32 {
    let mut num_of_words:i32 = 0;
    for c in s.chars().rev() {
        if c != ' ' {
            num_of_words += 1;
        } else if num_of_words > 0 {
            return num_of_words; 
        }
    }
    return num_of_words;
}
