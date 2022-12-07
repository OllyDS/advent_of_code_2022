use std::collections::{HashSet, VecDeque};

use crate::utils::read_file;

pub fn day_six_part_one() {
    let contents = read_file("./src/files/input_day_06.txt".to_string());

    let result = detect_packet_marker(contents);
    println!("{:?}", result);
}

/// finds the packet marker in O(n) time.
pub fn detect_packet_marker(contents: String) -> usize {
    // contains our last 4 unique elements
    let mut last_four: VecDeque<char> = VecDeque::new();
    // contains a list of the latest unique elements
    let mut seen: HashSet<char> = HashSet::new();

    // iterate through our string by char
    for (idx, c) in contents.chars().enumerate() {
        // check to see if the char exists in our set
        if seen.contains(&c) {
            // if it does we need to remove the first n elements up to the curr matching char
            for el in last_four.clone().into_iter() {
                if el != c {
                    // if the curr char is not our matching one, remove it from the start of our vec
                    last_four.pop_front();
                } else {
                    // if it matches, remove it (we add again later)
                    last_four.pop_front();
                    // then ensure our set matches our current last_four, so we can begin our search for unique clashes again.
                    seen = last_four.iter().map(|el| el.clone()).collect::<HashSet<_>>();
                    break;
                }
            }
        }

        // add the latest char
        seen.insert(c);
        last_four.push_back(c);


        // if our last_four vec is length 4, we have foudn our packet marker
        if last_four.len() == 4 {
            return idx + 1
        }
    }
    0
}