use hashbrown::HashSet;

fn move_rope(moves: &[(isize, isize, usize)], param: usize) -> usize {
    let mut rope = vec![(0, 0); param];
    let mut visited = HashSet::new();
    //start at 0,0
    visited.insert((0, 0));
    for &(moveX, moveY, distance) in moves {
        for _ in 0..distance {
            rope[0].0 += moveX;
            rope[0].1 += moveY;
            for i in 1..rope.len() {
                let (dx, dy) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    rope[i].0 += dx.signum();
                    rope[i].1 += dy.signum();
                }
            }
            visited.insert(rope[param - 1]);
        }
    }
    visited.len()
}

fn main() {
    let input = include_str!("rope.txt");
    let moves = input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(' ').unwrap();
            let (moveX, moveY) = match direction.as_bytes()[0] as char {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => unreachable!("Invalid direction"),
            };
            (moveX, moveY, distance.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    println!("Part 1: {}", move_rope(&moves, 2));
    println!("Part 2: {}", move_rope(&moves, 10));
}
