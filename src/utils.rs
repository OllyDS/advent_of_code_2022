use std::{fs, collections::HashMap};

/// takes a path for the location of the text file and converts to a String.
pub fn read_file(path: String) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
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