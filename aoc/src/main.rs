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

fn find_the_same_char(backpack: &Result<String, std::io::Error>) -> char {
    let backpack = backpack.as_ref().unwrap();
    let backpack_length = backpack.len();
    let (first, last) = backpack.split_at(backpack_length / 2);
    let mut found_char = ' ';
    for (l) in first.chars() {
        for (r) in last.chars() {
            if l == r {
                found_char = l;
            }
        }
    }
    found_char
}

fn backpack_to_int(found_item: char) -> i32 {
    match found_item as u8 {
        b'a'..=b'z' => (found_item as usize - b'a' as usize + 1)
            .try_into()
            .unwrap(),
        b'A'..=b'Z' => (found_item as usize - b'A' as usize + 27)
            .try_into()
            .unwrap(),
        _ => 0,
    }
}
fn main() {
    let file = read_lines("elves_backpacks.txt");
    match file {
        Ok(file) => {
            let lines = file.collect::<Vec<_>>();
            let d1 = lines
                .iter()
                .map(|line| find_the_same_char(line))
                .map(|line| backpack_to_int(line));
            println!("{:?}", d1.sum::<i32>());
        }
        Err(_) => {
            println!("error")
        }
    }
}
