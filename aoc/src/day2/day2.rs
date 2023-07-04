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
    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
    // A(1) - rock, B(2) - paper, C(3) - scissors
    // lost - 0, draw - 3, win - 6
    match line {
        "AY" => return 1 + 3,
        "AZ" => return 2 + 6,
        "AX" => return 3 + 0,
        "BY" => return 2 + 3,
        "BZ" => return 3 + 6,
        "BX" => return 1 + 0,
        "CY" => return 3 + 3,
        "CZ" => return 1 + 6,
        "CX" => return 2 + 0,
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
