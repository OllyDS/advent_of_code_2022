use aoc::day_04::calculate_num_fully_contained;
use aoc::utils::read_file;

#[test]
fn test_calculate_num_fully_contained() {
    let test_path = read_file("./src/files/test_day_04.txt".to_string());

    assert_eq!(calculate_num_fully_contained(test_path), 2);
}
