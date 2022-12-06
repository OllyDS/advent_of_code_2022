use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_file;

#[derive(Debug)]
pub struct Commands {
    _move: i32,
    _from: i32,
    _to: i32
}

pub fn day_five_part_one() {
    let contents = read_file("./src/files/input_day_05.txt".to_string());
    let (table, commands) = split_content(contents);

    let horizontal_table: HashMap<i32, Vec<String>> = convert_to_horizontal_table(table);

    let instructions: Vec<Commands> = convert_commands_to_succinct_struct(commands);

    let re_ordered_crates: HashMap<i32, Vec<String>> = reorder_crates_from_instructions(instructions, horizontal_table);
    
    let top_crates: Vec<String> = re_ordered_crates
        .into_iter()
        // sort by crate number
        .sorted_by_key(|entry| entry.0)
        .map(|map| map.1.first().unwrap().clone())
        .collect();
    
    let result = top_crates
        .into_iter()
        .map(|c| c.split(['[',']']).collect::<String>())
        .collect::<String>();
    
    println!("commands: {:?}", result);

}

/// will split the table from the commands into a tuple of str for easier handling
pub fn split_content(contents: String) -> (String, String) {
    contents
        .split("\n\n")
        .map(|el| el.to_string())
        .collect_tuple()
        .expect("There to only be a single instance of a double return")
}

/// converts vertical, columnar table into a hashmap of horizontal data.
/// this will make it easier to move objects from index to index
pub fn convert_to_horizontal_table(table: String) -> HashMap<i32, Vec<String>>{
    let mut split_table = table.split("\n").collect_vec();
    let num_columns = split_table
        .pop()
        .expect("to have captured the last row in the table")
        .trim()
        .split(" ")
        .last()
        .expect("to have captured the last element in the row, which should be an int in str format")
        .parse::<i32>()
        .expect("to parse the str to i32");

    let chunked_rows = split_table.into_iter().map(|row| {
        // iterate through each row and split into chunks based on column width (4)
        return row
            .chars()
            .into_iter()
            .collect_vec()
            .chunks(4)
            .collect_vec()
            // next convert the chars back into strings, so we have an array of n columns
            .iter()
            .map(|el| el.iter().collect::<String>())
            .collect_vec();
    }).collect_vec();

    let mut horizontal_table: HashMap<i32, Vec<String>> = HashMap::new();

    chunked_rows.into_iter().for_each(|row| {
        row.into_iter().enumerate().for_each(|(idx, el)| {
            let trimmed_el = el.trim().to_string();
            if !trimmed_el.is_empty() {
                let entry = horizontal_table.entry(((idx  as i32) % num_columns) + 1);
                entry.and_modify(|e| e.push(trimmed_el.clone())).or_insert_with(|| Vec::from([trimmed_el]));
            }
        });
    });

    return horizontal_table;
}

pub fn convert_commands_to_succinct_struct(commands: String) -> Vec<Commands> {
    commands
        .split("\n")
        .collect_vec()
        .into_iter().map(|command: &str| {
            let split_command: Vec<&str> = command.split(" ").collect_vec();
            return Commands {
                _move: split_command[1].parse::<i32>().expect("first command to be parsed to int"),
                _from: split_command[3].parse::<i32>().expect("second command to be parsed to int"),
                _to: split_command[5].parse::<i32>().expect("third command to be parsed to int"),
            };
        })
        .collect()
}

pub fn reorder_crates_from_instructions(instructions:  Vec<Commands>, mut hash_table: HashMap<i32, Vec<String>>) -> HashMap<i32, Vec<String>> {
    // re-organise crates according to the commands
    instructions.into_iter().for_each(|cmd| {
        let mut i = 0;
        while i < cmd._move {
            // fetch the top crate from the stack (first element) and remove it
            let _crate = hash_table.get_mut(&cmd._from).unwrap().remove(0);
            // insert the crate on to the top of the stack in the appropriate column
            hash_table.get_mut(&cmd._to).unwrap().insert(0, _crate.clone());
            i += 1;
        }
    });

    return hash_table;
}