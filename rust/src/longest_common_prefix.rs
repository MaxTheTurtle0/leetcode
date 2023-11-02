pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
    let mut longest_common_prefix = "".to_string();
    if strs.is_empty() {
        return longest_common_prefix;
    }
    strs.sort();
    for i in 0..strs[0].len() {
        if strs[0].as_bytes()[i] == strs[strs.len()-1].as_bytes()[i] {
            longest_common_prefix.push_str(strs[0].chars().nth(i).unwrap().to_string().as_str());
        } else {
            break;
        }
    }
    return longest_common_prefix;
}
