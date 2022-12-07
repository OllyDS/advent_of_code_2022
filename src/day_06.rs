use std::collections::{HashSet, VecDeque};

use crate::utils::read_file;

pub fn day_six_part_one() {
    let contents = read_file("./src/files/input_day_06.txt".to_string());

    let result = detect_packet_marker(contents, 4);
    println!("{:?}", result);
}

pub fn day_six_part_two() {
    let contents = read_file("./src/files/input_day_06.txt".to_string());

    let result = detect_packet_marker(contents, 14);
    println!("{:?}", result);
}

/// finds the packet marker in O(n) time.
pub fn detect_packet_marker(contents: String, n: usize) -> usize {
    // contains our last 4 unique elements
    let mut last_n_elements: VecDeque<char> = VecDeque::new();
    // contains a list of the latest unique elements
    let mut seen: HashSet<char> = HashSet::new();

    // iterate through our string by char
    for (idx, c) in contents.chars().enumerate() {
        // check to see if the char exists in our set
        if seen.contains(&c) {
            // if it does we need to remove the first n elements up to the curr matching char
            for el in last_n_elements.clone().into_iter() {
                if el != c {
                    // if the curr char is not our matching one, remove it from the start of our vec
                    last_n_elements.pop_front();
                } else {
                    // if it matches, remove it (we add again later)
                    last_n_elements.pop_front();
                    // then ensure our set matches our current last_n_elements, so we can begin our search for unique clashes again.
                    seen = last_n_elements.iter().map(|el| el.clone()).collect::<HashSet<_>>();
                    break;
                }
            }
        }

        // add the latest char
        seen.insert(c);
        last_n_elements.push_back(c);


        // if our last_n_elements vec is length n, we have found our packet marker
        if last_n_elements.len() == n {
            return idx + 1
        }
    }
    0
}
