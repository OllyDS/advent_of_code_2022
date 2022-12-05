use aoc::day_04::{day_four_part_one, day_four_part_two};

#[test]
fn test_day_four_part_one() {
    let test_path = Some("./src/files/test_day_04.txt".to_string());

    assert_eq!(day_four_part_one(test_path), 2);
}

#[test]
fn test_day_four_part_two() {
    let test_path = Some("./src/files/test_day_04.txt".to_string());

    assert_eq!(day_four_part_two(test_path), 4);
}
