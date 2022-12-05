use std::collections::HashMap;

use crate::utils::{create_alphabet_values, read_file};

pub fn day_three_calc_part_one() {
    let value_lookup: HashMap<String, usize> = create_alphabet_values();

    let contents = read_file("./src/files/input_day_03.txt".to_string());

    let sum_total = calculate_priority_values(contents, value_lookup);

    print!("{}", sum_total);
}

pub fn calculate_priority_values(contents: String, value_lookup: HashMap<String, usize>) -> usize {
    let mut priority_values: Vec<usize> = Vec::new();

    for line in contents.split("\n") {
        // hashmap to store unique str chars
        let mut char_map: HashMap<String, usize> = HashMap::new();

        // split the string into a tuple containing two strings of equal length
        let (left, right) = line.split_at(line.len() / 2);

        // add each char from the first string in the tuple to the char_map
        left
            .chars()
            .enumerate()
            .for_each(|(idx, c)| { char_map.insert(c.to_string(), idx); });

        // find the matching character from the tuple strings
        let priority = right
            .chars()
            .into_iter()
            .filter(|c| char_map.contains_key(&c.to_string()))
            .collect::<Vec<_>>()[0];

        // lookup the value associated with the matched character and store it in our hashmap
        priority_values.push(*value_lookup.get(&priority.to_string()).unwrap());
    }

    // return the sum of the values as i32
    priority_values.into_iter().sum::<usize>()
}

pub fn day_three_calc_part_two() {
    let value_lookup: HashMap<String, usize> = create_alphabet_values();

    let contents = read_file("./src/files/input_day_03.txt".to_string());
    let split_contents = contents.split("\n").into_iter().collect::<Vec<&str>>();
    let chunks = split_contents.chunks(3);

    let res = chunks.into_iter().map(|chunk| {
        for line in chunk[0].chars() {
            if chunk[1].contains(line) && chunk[2].contains(line) {
                return value_lookup.get(&line.to_string()).unwrap().clone();
            }
        }
        return 0;
    }).sum::<usize>();

    println!("{}", res);

}
