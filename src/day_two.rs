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
        ("AX", 3), // Rock -> Lose, Scissors, 0 + 3
        ("AY", 4), // Rock -> Draw, Rock, 3 + 1
        ("AZ", 8), // Rock -> Win, Paper, 6 + 2
        ("BX", 1), // Paper -> Lose, Rock, 0 + 1
        ("BY", 5), // Paper -> Draw, Paper, 3 + 2
        ("BZ", 9), // Paper -> Win, Scissors, 6 + 3
        ("CX", 2), // Scissors -> Lose, Paper, 0 + 2
        ("CY", 6), // Scissors -> Draw, Scissors, 3 + 3
        ("CZ", 7), // Scissors -> Win, Rock, 6 + 1
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
