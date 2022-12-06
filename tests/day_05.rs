use aoc::{day_05::{split_content}, utils::read_file};

#[test]
fn test_split_content() {
    let test_path = read_file("./src/files/test_day_05.txt".to_string());

    assert!(!split_content(test_path.clone()).0.is_empty());
    assert!(!split_content(test_path).1.is_empty());
}
