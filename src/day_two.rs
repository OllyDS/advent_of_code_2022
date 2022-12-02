use std::collections::HashMap;

use crate::utils::read_file;

pub fn day_two_calc() {
    let path = "./src/files/input_day_two.txt".to_string();
    let total_score = calculate_score(path);
    println!("{:?}", total_score);
}

pub fn calculate_score(path: String) -> i32 {
    let content = read_file(path);

    let strategy = HashMap::from([
        ("AX", 4), // Rock, Rock, 1 + 3
        ("AY", 8), // Rock, Paper, 2 + 6
        ("AZ", 3), // Rock, Scissors, 3 + 0
        ("BX", 1), // Paper, Rock, 1 + 0
        ("BY", 5), // Paper, Paper, 2 + 3
        ("BZ", 9), // Paper, Scissors, 3 + 6
        ("CX", 7), // Scissors, Rock, 1 + 6
        ("CY", 2), // Scissors, Paper, 2 + 0
        ("CZ", 6), // Scissors, Scissors, 3 + 3
    ]);

    let moves = content.split("\n").collect::<Vec<_>>();

    let scores = moves
        .into_iter()
        .map(|m| {
            let key = m.split(" ").collect::<Vec<_>>().join("");
            return *strategy.get(key.as_str()).unwrap();
        })
        .collect::<Vec<_>>();

    scores.iter().sum()
}
