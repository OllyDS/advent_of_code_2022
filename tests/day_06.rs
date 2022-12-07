use aoc::{day_06::{detect_packet_marker}, utils::read_file};

#[test]
fn test_detect_packet_marker() {
    let test_path = read_file("./src/files/test_day_06.txt".to_string());

    assert_eq!(detect_packet_marker(test_path), 5);
    assert_eq!(detect_packet_marker(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 6);
    assert_eq!(detect_packet_marker(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
}
