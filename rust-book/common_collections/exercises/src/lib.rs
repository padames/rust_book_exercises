use std::collections::HashMap;

// 1. Given a list of integers, use a vector and return the median (when
// sorted, the value in the middle position) and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.


pub fn median_mode(input_data: &[i64]) -> Option<(f64, i64)> {
    if input_data.is_empty() {
        return None;
    }

    let mut values = input_data.to_vec();
    values.sort_unstable();

    let mid = values.len() / 2;
    let median = if values.len() % 2 == 0 {
        (values[mid - 1] as f64 + values[mid] as f64) / 2.0
    } else {
        values[mid] as f64
    };

    let mut counts = HashMap::new();
    for &value in input_data {
        *counts.entry(value).or_insert(0) += 1;
    }

    let mode = counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0; // the value that corresponds to the max count

    Some((median, mode))
}



// 2. Convert strings to pig latin. The first consonant of each word is moved
// to the end of the word and ay is added, so first becomes irst-fay. Words
// that start with a vowel have hay added to the end instead (apple becomes
// apple-hay). Keep in mind the details about UTF-8 encoding!



// 3. Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company; for example, “Add
// Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
// a list of all people in a department or all people in the company by
// department, sorted alphabetically.


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
