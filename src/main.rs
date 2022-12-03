use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn main() -> Result<(), Error> {
    parse_file(File::open("input")?);
    Ok(())
}

fn parse_file<R: Read>(input: R) {
    let br = BufReader::new(input);
    for line in br.lines() {
        // returns String
        let str_line = line.expect("Failed to read line");
        let vec = string_to_vector(str_line);
        println!("{:?}", vec);
    }
}

fn string_to_vector(line: String) -> Vec<char> {
    let mut iter = line.chars();
    let mut vec: Vec<char> = Vec::new();
    for _ in 0..3 {
        vec.push(iter.next().unwrap());
    }
    vec
}
