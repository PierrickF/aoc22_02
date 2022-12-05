use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn main() -> Result<(), Error> {
    parse_file(File::open("input")?);
    Ok(())
}

fn parse_file<R: Read>(input: R) {
    let mut final_score: u32 = 0;
    let br = BufReader::new(input);
    for line in br.lines() {
        let str_line = line.expect("Failed to read line");
        let vec = string_to_vector(str_line);
        final_score = score_round(vec) + final_score;
        println!("\n");
    }
    println!("{final_score}");
}

fn string_to_vector(line: String) -> Vec<char> {
    let mut iter = line.chars();
    let mut vec: Vec<char> = Vec::new();
    for _ in 0..3 {
        vec.push(iter.next().unwrap());
    }
    vec
}

fn score_round(vec: Vec<char>) -> u32 {
    let mut score: u32 = 0;

    match vec[2] {
        'X' => {
            score += 1;
            println!("1 point for playing rock");
        }
        'Y' => {
            score += 2;
            println!("2 points for playing paper");
        }
        'Z' => {
            score += 3;
            println!("3 points for playing scissors");
        }
        _ => panic!("uh oh"),
    }

    if vec[2] == 'X' && vec[0] == 'A' {
        score += 3;
        println!("3 points for draw");
    } else if vec[2] == 'Y' && vec[0] == 'B' {
        score += 3;
        println!("3 points for draw");
    } else if vec[2] == 'Z' && vec[0] == 'C' {
        score += 3;
        println!("3 points for draw");
    } else if vec[2] == 'X' && vec[0] == 'C' {
        score += 6;
        println!("6 points for win");
    } else if vec[2] == 'Y' && vec[0] == 'A' {
        score += 6;
        println!("6 points for win");
    } else if vec[2] == 'Z' && vec[0] == 'B' {
        score += 6;
        println!("6 points for win");
    }

    score
}
