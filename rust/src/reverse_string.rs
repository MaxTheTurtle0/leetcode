pub fn reverse_string(s: &mut Vec<char>) {
    for i in 0..s.len() / 2 {
        let tmp = s[i];
        let other_index = s.len() - 1 - i;
        s[i] = s[other_index];
        s[other_index] = tmp;
    }
}
