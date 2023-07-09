use std::path::PathBuf;

use hashbrown::{HashMap, HashSet};

fn main() {
    let input = include_str!("osinstructions.txt");
    let mut files = HashMap::<_, HashSet<_>>::new();
    let mut pwd = PathBuf::new();
    for line in input.split('$').skip(1) {
        match line.trim().lines().next().unwrap() {
            "ls" => {
                let entries = line.lines().skip(1).map(|line| {
                    let (size, f) = line.split_once(' ').unwrap();
                    (size.parse::<i64>().unwrap_or(-1), f)
                });
                files.entry(pwd.clone()).or_default().extend(entries);
            }
            "cd .." => {
                pwd.pop();
            }
            cd_dir => pwd.push(cd_dir.split_once(' ').unwrap().1),
        }
    }
    println!("{:#?}", files)
}
