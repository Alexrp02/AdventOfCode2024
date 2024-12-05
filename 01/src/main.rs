use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_from_file<P>(filename: P) -> (Vec<i32>, Vec<i32>)
where
    P: AsRef<Path>,
    P: Display,
{
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(error) => panic!("Couldn't open the file {} : {}", filename, error),
    };
    let file_lines = BufReader::new(file)
        .lines()
        .flatten();

    let mut left_vector: Vec<i32> = vec![];
    let mut right_vector: Vec<i32> = vec![];

    for line in file_lines {
        let split: Vec<&str> = line.split("   ").collect();
        if split.len() != 2 {
            panic!(
                "There is a line that doesn't have exactly 2 elements: {}",
                line
            );
        }
        left_vector.push(split[0].parse().unwrap());
        right_vector.push(split[1].parse().unwrap());
    }

    (left_vector, right_vector)
}

fn main() {
    let filename = String::from("input.txt");
    println!("Reading from the input file...");
    let mut vector_pair = read_from_file(&filename);
    vector_pair.0.sort();
    vector_pair.1.sort();

    let mut sum = 0;
    for i in 0..vector_pair.0.len() {
        sum += (vector_pair.0[i] - vector_pair.1[i]).abs();
    }

    println!("The total difference between the two columns is {sum}");
}
