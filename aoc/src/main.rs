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

fn main() {
    let lines = read_lines("elves_foods.txt");
    let mut temp = 0;
    let mut output = [0; 3];
    match lines {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        if !line.is_empty() {
                            temp = temp + line.parse::<i32>().unwrap();
                        } else {
                            if (output.iter().any(|&x| temp > x)) {
                                output.sort();
                                output[0] = temp;
                            }
                            temp = 0;
                        }
                    }
                    Err(_) => println!("Error"),
                }
            }
        }
        Err(_) => println!("File not found"),
    }
    println!("{:?}", output.iter().sum::<i32>())
}
