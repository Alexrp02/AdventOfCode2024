use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_sequences_from_file(filename: &String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    let file = match File::open(Path::new(&filename)) {
        Ok(file) => file,
        Err(e) => panic!("There was an error: {}", e),
    };

    let file_lines = BufReader::new(file).lines().flatten();

    for line in file_lines {
        let line_content: Vec<i32> = line
            .split(" ")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        result.push(line_content);
    }

    result
}

fn count_safe_sequences(sequences: &Vec<Vec<i32>>) -> i32 {
    let mut total_safe_sequences = 0;
    for sequence in sequences {
        if check_sequence(sequence, false) {
            total_safe_sequences += 1;
        }
    }

    total_safe_sequences
}

fn check_sequence(sequence: &Vec<i32>, deleted_one_interval: bool) -> bool {
    if sequence.len() == 1 {
        return true;
    }

    let is_decreasing = check_decreasing(sequence);
    let mut result = true;

    for (i, number) in sequence.iter().enumerate() {
        if i != sequence.len() - 1 {
            let next_number = sequence[i + 1];
            let difference = next_number - number;

            if !(difference >= -3 && difference <= -1) && is_decreasing {
                if !deleted_one_interval {
                    let mut clone = sequence.clone();
                    clone.remove(i);
                    result = check_sequence(&clone, true);
                } else {
                    result = false;
                }
            } else if !(difference >= 1 && difference <= 3) && !is_decreasing {
                if !deleted_one_interval {
                    let mut clone = sequence.clone();
                    clone.remove(i);
                    result = check_sequence(&clone, true);
                } else {
                    result = false;
                }
            }
        }
    }

    if !deleted_one_interval && !result{
        let mut clone = sequence.clone();
        clone.remove(sequence.len() - 1);
        result = check_sequence(&clone, true);
    }

    result
}

fn check_decreasing(sequence: &[i32]) -> bool {
    let mut increasing = 0;
    let mut decreasing = 0;
    for (i, num) in sequence.iter().enumerate() {
        if i != sequence.len() - 1 {
            if num > &sequence[i + 1] {
                decreasing += 1
            } else {
                increasing += 1
            }
        }
    }
    return decreasing > increasing;
}

fn main() {
    let filename = "input.txt".to_string();
    let sequences: Vec<Vec<i32>> = read_sequences_from_file(&filename);

    let safe_sequences_count = count_safe_sequences(&sequences);

    println!("The number of safe sequences is {}", safe_sequences_count);
}
