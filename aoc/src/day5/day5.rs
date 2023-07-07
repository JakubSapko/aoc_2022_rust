use itertools::Itertools;

fn five_one(mut stacks: Vec<Vec<char>>, operations: &Vec<(usize, usize, usize)>) -> String {
    for &(count, from, to) in operations {
        for _ in 0..count {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }
    stacks.iter().map(|stack| stack.last().unwrap()).join("")
}

fn five_two(mut stacks: Vec<Vec<char>>, operations: &Vec<(usize, usize, usize)>) -> String {
    for &(count, from, to) in operations {
        let to_prolong = stacks[to - 1].len() + count;
        stacks[to - 1].resize(to_prolong, ' ');
        for i in 0..count {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1][to_prolong - 1 - i] = item;
        }
    }
    stacks.iter().map(|stack| stack.last().unwrap()).join("")
}
fn main() {
    let file = include_str!("stacks.txt");
    let (boxes, operations) = file.split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    for line in boxes.lines().rev().skip(1).map(str::as_bytes) {
        for i in 0..stacks.len() {
            let character = line[i * 4 + 1];
            if character.is_ascii_alphabetic() {
                stacks[i].push(character as char);
            }
        }
    }
    let operations = operations
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|op| op.parse::<usize>().ok())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let fo = five_one(stacks.clone(), &operations);
    let ft = five_two(stacks, &operations);
    println!("{}", fo);
    println!("{}", ft);
}
