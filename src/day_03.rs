use std::collections::HashMap;

use crate::utils::read_file;

pub fn day_three_calc() {
    let value_lookup: HashMap<String, usize> = create_alphabet_values();

    let contents = read_file("./src/files/test_day_03.txt".to_string());

    let sum_total = calculate_priority_values(contents, value_lookup);

    print!("{}", sum_total);
}

/// The following code block handles creating a range of chars from a-z and A-Z
/// and assigning them values from 1 - 52
pub fn create_alphabet_values() -> HashMap<String, usize> {
    let mut value_lookup: HashMap<String, usize> = HashMap::new();
    let alphabet_lower = 'a'..='z';

    for (idx, character) in alphabet_lower.enumerate() {
        value_lookup.insert(character.to_string(), idx + 1);
        value_lookup.insert(character.to_uppercase().collect::<String>(), idx + 27);
    }

    return value_lookup;
}

pub fn calculate_priority_values(contents: String, value_lookup: HashMap<String, usize>) -> i32 {
    let mut priority_values: Vec<usize> = Vec::new();

    for line in contents.split("\n") {
        // hashmap to store unique str chars
        let mut char_map: HashMap<String, usize> = HashMap::new();

        // split the string into a tuple containing two strings of equal length
        let split_line = line.split_at(line.len() / 2);

        // add each char from the first string in the tuple to the char_map
        split_line
            .0
            .split("")
            .enumerate()
            .for_each(|(idx, c)| { char_map.insert(c.to_string(), idx); });

        // find the matching character from the tuple strings
        let priority = split_line
            .1
            .split("")
            .into_iter()
            .filter(|c| char_map.contains_key(&c.to_string()) && !c.is_empty())
            .collect::<Vec<_>>()[0];

        // lookup the value associated with the matched character and store it in our hashmap
        priority_values.push(*value_lookup.get(&priority.to_string()).unwrap());
    }

    println!("{:?}", priority_values);
    // return the sum of the values as i32
    priority_values
        .into_iter()
        .reduce(|a, c| a + c)
        .expect("Sum of values to be returned")
        .try_into()
        .unwrap()
}
