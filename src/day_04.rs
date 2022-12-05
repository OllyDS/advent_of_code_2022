use itertools::Itertools;

use crate::utils::read_file;

pub fn day_four_part_one(test_contents: Option<String>) -> i32 {
    let contents: String = read_file(if test_contents.is_some() { test_contents.unwrap() } else {"./src/files/input_day_04.txt".to_string() });
    let groups = group_into_tuples(contents);
    
    calculate_num_fully_contained(groups)
}

pub fn day_four_part_two(test_contents: Option<String>) -> i32 {
    let contents: String = read_file(if test_contents.is_some() { test_contents.unwrap() } else {"./src/files/input_day_04.txt".to_string() });
    let groups = group_into_tuples(contents);

    calc_num_overlapping(groups)
}

pub fn group_into_tuples(contents: String) -> Vec<((i32, i32), (i32, i32))> {
    let groups: Vec<&str> = contents.split("\n").collect_vec();

    groups.into_iter().map(|g| {
        // convert the strings into ints
        let vec_ints: Vec<i32> = g
            .split(&[',', '-'])
            .map(|int| int.parse::<i32>().expect("to parse str into i32"))
            .collect_vec();
        // group the pairs into tuples, representing each elf in the pair
        return ((vec_ints[0], vec_ints[1]), (vec_ints[2], vec_ints[3]));
    }).collect_vec()
}

pub fn calculate_num_fully_contained(tuples: Vec<((i32, i32), (i32, i32))>) -> i32 {
    tuples.into_iter().map(|(left, right) | {
        if is_contained_within(left, right) { 1 } else { 0 }
    }).sum()
}

/// used in part 01 to check whether the range of one tuple completely overlaps the other
/// so that it could entirely contain it's range
pub fn is_contained_within(left: (i32, i32), right: (i32, i32)) -> bool {
    let left_range = left.0..=left.1;
    let right_range = right.0..=right.1;

    (left_range.contains(&right.0) && left_range.contains(&right.1)) ||
    (right_range.contains(&left.0) && right_range.contains(&left.1))
}

pub fn calc_num_overlapping(tuples: Vec<((i32, i32), (i32, i32))>) -> i32 {
    tuples.into_iter().map(|(left, right) | {
        if is_overlap(left, right) { 1 } else { 0 }
    }).sum()
}

/// used in part 02 to check whether there is any overlap at all
pub fn is_overlap(left: (i32, i32), right: (i32, i32)) -> bool {
    left.0 <= right.1 && left.1 >= right.0
}