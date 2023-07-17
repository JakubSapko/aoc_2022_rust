fn map_to_visible(trees: Vec<Vec<u8>>) -> u32 {
    // if a tree is located on the edges of the map, it is visible
    // otherwise it is only visible if all the trees in any of the 4 directions are lower than it is
    let outer_rows = trees.get(0).unwrap().len() * 2;
    let outer_cols = (trees.len() - 2) * 2;

    let mut visible = (outer_rows + outer_cols) as u32;

    for (y, row) in map_out_edges(&trees)
        .iter()
        .enumerate()
        .map(|(i, row)| (i + 1, row))
    {
        for (x, col) in row.iter().enumerate().map(|(j, col)| (j + 1, col)) {
            let tree_size = **col;
            let visible_bottom = (y..trees.len() - 1).all(|v| trees[v + 1][x] < tree_size);
            let visible_top = (1..=y).all(|v| trees[v - 1][x] < tree_size);
            let visible_left = (1..=x).all(|v| trees[y][v - 1] < tree_size);
            let visible_right = (x..=row.len()).all(|v| trees[y][v + 1] < tree_size);
            if visible_bottom || visible_top || visible_left || visible_right {
                visible += 1;
            }
        }
    }
    visible
}

fn map_out_edges(input: &Vec<Vec<u8>>) -> Vec<Vec<&u8>> {
    input[1..input.len() - 1]
        .iter()
        .map(|row| row[1..row.len() - 1].iter().collect())
        .collect::<Vec<Vec<_>>>()
}
fn main() {
    let input = include_str!("trees.txt");
    let test = include_str!("test.txt");
    println!("{}", input);
    let trees: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let visible = map_to_visible(trees);
    println!("{:#?}", visible);

    let test_data = test
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{:#?}", test_data);
    let visible_test = map_to_visible(test_data);
    println!("{:#?}", visible_test);
}
