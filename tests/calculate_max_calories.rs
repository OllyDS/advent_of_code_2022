use aoc::day_one::calculate_max_calories;

use std::fs;

#[test]
fn test_calculate_max_calories() {
    // read txt file as string
    let test_contents = fs::read_to_string("./src/files/test_day_one.txt")
        .expect("Should have been able to read the file");

    assert_eq!(calculate_max_calories(test_contents), 2400);
}
