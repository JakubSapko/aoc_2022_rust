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
}
