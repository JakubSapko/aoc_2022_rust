use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn determine_score(line: &str) -> i32 {
    match line {
        "AY" => return 6 + 2,
        "BZ" => return 6 + 3,
        "CX" => return 6 + 1,
        "AX" => return 3 + 1,
        "BY" => return 3 + 2,
        "CZ" => return 3 + 3,
        "AZ" => return 0 + 3,
        "BX" => return 0 + 1,
        "CY" => return 0 + 2,
        _ => return 0,
    };
}

fn main() {
    let lines = read_lines("rockpaperscissors.txt");
    match lines {
        Ok(lines) => {
            let values: i32 = lines
                .into_iter()
                .map(|line| line.unwrap().split_whitespace().collect::<String>())
                .map(|line| determine_score(&line))
                .sum();
            println!("{}", values)
        }
        Err(_) => {}
    }
}
