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
        .0; // the key that corresponds to the max count value

    Some((median, mode))
}



// 2. Convert strings to pig latin. The first consonant of each word is moved
// to the end of the word and ay is added, so first becomes irst-fay. Words
// that start with a vowel have hay added to the end instead (apple becomes
// apple-hay). Keep in mind the details about UTF-8 encoding!


pub fn sentence_to_pig_latin(sentence: &str) -> String {
    let mut sentence_pig_latin = String::new();

    for w in sentence.to_string().split_whitespace() {
        if let Some(word) = word_to_pig_latin(w) {
            sentence_pig_latin.push_str(&word);
        }
    }
    sentence_pig_latin
}



fn word_to_pig_latin(original_word: &str) -> Option<String> {
    let vowels = [ 'a', 'e', 'i', 'o', 'u'];

    let mut word_iter = original_word.chars();

    if let Some(ch) = word_iter.next() { 
        if vowels.contains(&ch) {
            return Some(original_word.to_string() + "-hay");
        }
        else {
            let mut piggy_latin_word = String::new();
            let mut next_ch = word_iter.next();
            while next_ch != None  {
                if let Some(c) = next_ch {
                    piggy_latin_word.push(c);
                }
                next_ch = word_iter.next();
            }
            return Some(piggy_latin_word + "-fay");
        }
    }
    None
}




// 3. Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company; for example, “Add
// Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
// a list of all people in a department or all people in the company by
// department, sorted alphabetically.



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_to_pig_latin_when_first_letter_is_a_vowel() {
        
        let input_word = "adam";
        let expected_word = "adam-hay".to_string();
        assert_eq!( word_to_pig_latin(input_word), Some(expected_word));
    }

    #[test]
    fn word_to_pig_latin_when_first_letter_is_not_a_vowel() {

        let input_word = "garden";
        let expected_word = "arden-fay".to_string();
        assert_eq!( word_to_pig_latin(input_word), Some(expected_word));
    }

    #[test]
    fn word_to_pig_latin_returns_none_for_empty_string() {
        assert_eq!( word_to_pig_latin(""), None);
    }
}
