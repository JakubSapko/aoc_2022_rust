use itertools::Itertools;
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

fn check_for_containing(tupl: (String, String)) -> bool {
    let (first, second) = tupl;
    let first_range = first.split('-').collect_tuple::<(&str, &str)>().unwrap();
    let second_range = second.split('-').collect_tuple::<(&str, &str)>().unwrap();
    let first_min = first_range.0.parse::<i32>().unwrap();
    let first_max = first_range.1.parse::<i32>().unwrap();
    let second_min = second_range.0.parse::<i32>().unwrap();
    let second_max = second_range.1.parse::<i32>().unwrap();
    let first_contains_second = first_min <= second_min && first_max >= second_max;
    let second_contains_first = second_min <= first_min && second_max >= first_max;
    if first_contains_second || second_contains_first {
        return true;
    }
    return false;
}

fn check_for_overlapping(tupl: (String, String)) -> bool {
    let (first, second) = tupl;
    let first_range = first.split('-').collect_tuple::<(&str, &str)>().unwrap();
    let second_range = second.split('-').collect_tuple::<(&str, &str)>().unwrap();
    let first_min = first_range.0.parse::<i32>().unwrap();
    let first_max = first_range.1.parse::<i32>().unwrap();
    let second_min = second_range.0.parse::<i32>().unwrap();
    let second_max = second_range.1.parse::<i32>().unwrap();
    let second_overlaps_first = second_min <= first_max && second_max >= first_min;
    let first_overlaps_second = first_min <= second_max && first_max >= second_min;
    if first_overlaps_second || second_overlaps_first {
        return true;
    }
    return false;
}
fn main() {
    let file = read_lines("elves_cleaning_pairs.txt");
    match file {
        Ok(lines) => {
            // let pairs = lines
            //     .map(|line| line.unwrap())
            //     .map(|line: String| {
            //         let parts: Vec<&str> = line.split(',').collect();
            //         (parts[0].to_owned(), parts[1].to_owned())
            //     })
            //     .map(|tuple| check_for_containing(tuple))
            //     .filter(|&value| value)
            //     .collect::<Vec<_>>()
            //     .len();
            let overlaps = lines
                .map(|line| line.unwrap())
                .map(|line: String| {
                    let parts: Vec<&str> = line.split(',').collect();
                    (parts[0].to_owned(), parts[1].to_owned())
                })
                .map(|tuple| check_for_overlapping(tuple))
                .filter(|&value| value)
                .collect::<Vec<_>>()
                .len();
            // println!("{} pairs contain each other", pairs);
            println!("{} pairs overlap", overlaps);
        }
        Err(err) => {
            println!("Error reading file: {}", err)
        }
    }
}
