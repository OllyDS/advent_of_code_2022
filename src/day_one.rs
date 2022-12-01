use std::fs;

pub fn day_one_calc() {
    // read txt file as string
    let contents = fs::read_to_string("./src/files/input_day_one.txt")
        .expect("Should have been able to read the file");

    let sum = calculate_max_calories(contents);
    println!("{:?}", sum);
}

/// function used to solve the problem from Day 01.
pub fn calculate_max_calories(contents: String) -> i32 {
    // split the string, grouping by newline
    let parsed_text: Vec<&str> = contents.split("\n").collect::<Vec<_>>();
    let mut sum_vecs: Vec<i32> = Vec::new();
    let mut curr: i32 = 0;

    for text in parsed_text {
        if !text.is_empty() {
            let int = text.parse::<i32>().expect("to have been a valid int");
            curr += int;
        } else {
            sum_vecs.push(curr);
            curr = 0;
        }
    }

    // sort the vec from smallest to largest
    sum_vecs.sort();

    let first = sum_vecs.pop().unwrap();
    let second = sum_vecs.pop().unwrap();
    let third = sum_vecs.pop().unwrap();

    return first + second + third;
}
