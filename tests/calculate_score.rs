use aoc::day_02::calculate_score;

#[test]
fn test_calculate_score() {
    let test_path = "./src/files/test_day_02.txt".to_string();

    assert_eq!(calculate_score(test_path), 12);
}
