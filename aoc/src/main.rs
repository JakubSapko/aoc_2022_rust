enum Operation {
    Noop,
    Add(i32),
}

fn solve_part_one(operations: &Vec<Operation>) -> i32 {
    let mut acc = 1;
    let mut signal_strength = 0;
    let mut current_cycle = 0;
    let breakpoints = vec![20, 60, 100, 140, 180, 220];

    for operation in operations {
        let cycles_to_wait = match operation {
            Operation::Noop => 1,
            Operation::Add(_) => 2,
        };

        for _ in 0..cycles_to_wait {
            current_cycle += 1;
            if breakpoints.contains(&current_cycle) {
                signal_strength += acc * current_cycle;
            }
        }

        if let Operation::Add(value) = operation {
            acc += value;
        }
    }
    signal_strength
}

fn solve_part_two(operations: &Vec<Operation>) -> String {
    let mut acc = 1;
    let mut current_cycle = 0;
    //40x6 pixels = 240
    let mut crt = vec!["."; 240];
    //sprite is 3 pixels wide
    let mut sprite = 0..=2;

    for operation in operations {
        let cycles_to_wait = match operation {
            Operation::Noop => 1,
            Operation::Add(_) => 2,
        };

        for _ in 0..cycles_to_wait {
            //sprite is on the pixel
            if sprite.contains(&(current_cycle % 40)) {
                crt[current_cycle] = "#";
            }
            current_cycle += 1;
        }

        if let Operation::Add(value) = operation {
            acc += value;
            sprite = if acc < 1 {
                0..=0
            } else {
                (acc as usize - 1)..=(acc as usize + 1)
            };
        }
    }
    crt.join("")
}
fn parse_input(input: &str) -> Vec<Operation> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| match l {
            "noop" => Operation::Noop,
            _ => Operation::Add(l.split_once(' ').unwrap().1.parse::<i32>().unwrap()),
        })
        .collect()
}
fn main() {
    let input = include_str!("cpu_one.txt");
    let operations = parse_input(input);
    let signal_strength = solve_part_one(&operations);
    println!("Part 1: {}", signal_strength);
    let crt = solve_part_two(&operations);
    println!("Part 2:");
    for chunk in crt.chars().collect::<Vec<_>>().chunks(40) {
        println!("{}", chunk.iter().collect::<String>());
    }
}
