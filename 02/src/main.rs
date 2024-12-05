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
        if check_sequence(sequence) {
            total_safe_sequences += 1;
        }
    }

    total_safe_sequences
}

fn check_sequence(sequence: &Vec<i32>) -> bool {
    if sequence.len() == 1 {
        return true;
    }
    if sequence[1] - sequence[0] == 0 {
        return false;
    }

    let is_decreasing = sequence[1] - sequence[0] < 0;

    for (i, number) in sequence.iter().enumerate() {
        if i != sequence.len() - 1 {
            let next_number = sequence[i + 1];
            let difference = next_number - number;

            if is_decreasing {
                if difference > -1 || difference < -3 {
                    return false;
                }
            } else {
                if difference < 1 || difference > 3 {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    let filename = "input.txt".to_string();
    let sequences: Vec<Vec<i32>> = read_sequences_from_file(&filename);

    let safe_sequences_count = count_safe_sequences(&sequences);

    println!("The number of safe sequences is {}", safe_sequences_count);
}
