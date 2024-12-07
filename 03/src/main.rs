use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use regex::Regex;

fn get_text_from_filename(filename: &String) -> String {
    let file = File::open(Path::new(filename)).expect("The file couldn't be opened");

    let text: &mut String = &mut String::from("");
    let _ = BufReader::new(file).read_to_string(text);

    text.to_string()
}

fn main() {
    let input_text = get_text_from_filename(&String::from("input.txt"));

    let regex = Regex::new(r"mul\(\d+,\d+\)").expect("Error creating the regex");

    let array_of_match = get_array_of_match_from_regex(input_text, regex);

    process_matches_of_mul(array_of_match);
}

fn get_array_of_match_from_regex(input_text: String, regex: Regex) -> Vec<String> {
    let result = regex.find_iter(input_text.as_str()).map(|m| String::from(m.as_str())).collect(); 

    println!("{:?}", result);
    result
}

fn process_matches_of_mul(array_of_match: Vec<String>) {
    let re = Regex::new(r"\((\d+),(\d+)\)").expect("Error creating the regex when processing the mul");

    let mut result:i32 = 0;

    for m in array_of_match {
        if let Some(capture) = re.captures(m.as_str()) {
            let first_digit = capture[1].parse::<i32>().unwrap();
            let second_digit = capture[2].parse::<i32>().unwrap();
            println!("Multiplying {} with {}", first_digit, second_digit);
            result += first_digit * second_digit;
        }
    }

    println!("Result is {result}");
}
