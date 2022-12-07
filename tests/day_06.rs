use aoc::{day_06::{detect_packet_marker}, utils::read_file};

#[test]
fn test_detect_packet_marker() {
    let test_path = read_file("./src/files/test_day_06.txt".to_string());

    assert_eq!(detect_packet_marker(test_path, 4), 5);
    assert_eq!(detect_packet_marker(String::from("nppdvjthqldpwncqszvftbrmjlhg"), 4), 6);
    assert_eq!(detect_packet_marker(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 4), 10);
}

#[test]
fn test_detect_message_marker() {
    assert_eq!(detect_packet_marker(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 14), 19);
    assert_eq!(detect_packet_marker(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 14), 23);
}
