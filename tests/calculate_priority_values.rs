use std::collections::HashMap;

use aoc::day_03::{create_alphabet_values,calculate_priority_values};
use aoc::utils::read_file;

#[test]
fn test_calculate_priority_values() {
    let test_value_lookup: HashMap<String, usize> = create_alphabet_values();
    let test_path = read_file("./src/files/test_day_03.txt".to_string());

    assert_eq!(calculate_priority_values(test_path, test_value_lookup), 157);
}
