use aoc::day_01::calculate_max_calories;
use aoc::utils::read_file;

#[test]
fn test_calculate_max_calories() {
    // read txt file as string
    let test_contents = read_file("./src/files/test_day_01.txt".to_string());

    assert_eq!(calculate_max_calories(test_contents), 2400);
}
