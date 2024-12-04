use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn read_from_file<P>(filename: P) -> (Vec<u32>, Vec<u32>)
where
    P: AsRef<Path>,
    P: Display,
{
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(error) => panic!("Couldn't open the file {} : {}", filename, error),
    };
    let file_lines = BufReader::new(file).lines();

    let mut left_vector: Vec<u32> = vec![];
    let mut right_vector: Vec<u32> = vec![];

    for line in file_lines.flatten() {
        let split: Vec<&str> = line.split("   ").collect();
        if split.len() != 2 {
            panic!(
                "There is a line that doesn't have exactly 2 elements: {}",
                line
            );
        }
        println!("{:?}", split);
        left_vector.push(split[0].parse().unwrap());
        right_vector.push(split[1].parse().unwrap());
    }

    (left_vector, right_vector)
}

fn main() {
    let filename = String::from("input.txt");
    println!("Reading from the input file...");
    let vector_pair = read_from_file(&filename);
    println!("The right vector is: {:?}", vector_pair.0);
}
