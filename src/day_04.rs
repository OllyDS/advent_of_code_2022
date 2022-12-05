use itertools::Itertools;

use crate::utils::read_file;

pub fn day_four_part_one() {
    let contents: String = read_file("./src/files/input_day_04.txt".to_string());
    let results = calculate_num_fully_contained(contents);
    print!("{:?}", results);
}

pub fn calculate_num_fully_contained(contents: String) -> i32 {
    let groups: Vec<&str> = contents.split("\n").collect_vec();

    let int_tuples = groups.into_iter().map(|g| {
        // convert the strings into ints
        let vec_ints = g.split(&[',', '-']).map(|int| int.parse::<i32>().expect("to parse str into i32")).collect::<Vec<i32>>();
        // group the pairs into tuples, representing each elf in the pair
        return ((vec_ints[0], vec_ints[1]), (vec_ints[2], vec_ints[3]));
    }).collect_vec();

    int_tuples.into_iter().map(|(left, right) | {
        if is_contained_within(left, right) { 1 } else { 0 }
    }).sum()
}

pub fn is_contained_within(left: (i32, i32), right: (i32, i32)) -> bool {
    let left_range = left.0..=left.1;
    let right_range = right.0..=right.1;

    (left_range.contains(&right.0) && left_range.contains(&right.1)) ||
    (right_range.contains(&left.0) && right_range.contains(&left.1))
}
