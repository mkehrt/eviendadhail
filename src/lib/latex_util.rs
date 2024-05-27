use std::cmp::Ordering;

pub fn string_to_base_string(string: &String) -> String {
    string
        .chars()
        .filter(|c| char::is_alphabetic(*c))
        .collect::<String>()
}

pub fn compare_by_base_string(left: &String, right: &String) -> Option<Ordering> {
    let left_base = string_to_base_string(left);
    let right_base = string_to_base_string(right);

    left_base.partial_cmp(&right_base)
}
