use std::path::PathBuf;

use hashbrown::{HashMap, HashSet};

fn calculate_dir_size(
    key: &PathBuf,
    files: &HashMap<PathBuf, HashSet<(i64, &str)>>,
    dir_sizes: &mut HashMap<PathBuf, i64>,
) {
    if dir_sizes.contains_key(key) {
        return;
    }
    let size = files[key]
        .iter()
        .map(|&(s, d)| match s {
            -1 => {
                let dir = key.join(d);
                calculate_dir_size(&dir, files, dir_sizes);
                dir_sizes[&dir]
            }
            s => s,
        })
        .sum();
    dir_sizes.insert(key.clone(), size);
}

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
    println!("{:#?}", files);
    let mut dir_sizes = HashMap::new();
    for key in files.keys() {
        calculate_dir_size(key, &files, &mut dir_sizes);
    }
    let size = dir_sizes[&PathBuf::from("/")];
    println!("{}", size);
    let p1 = dir_sizes.values().filter(|&&s| s <= 100000).sum::<i64>();
    println!("{}", p1);
    let p2 = dir_sizes
        .values()
        .filter(|&&s| 40000000 + s >= size)
        .min()
        .copied()
        .unwrap();
    println!("{}", p2)
}
