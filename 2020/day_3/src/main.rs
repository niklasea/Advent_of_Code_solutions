#[derive(PartialEq, Eq)]
enum Square {
    Open,
    Tree,
}

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input file is not valid UTF-8.");
    let map: Vec<Vec<Square>> = input.lines()
        .map(|line| 
            line.chars()
                .map(|c| match c {
                    '.' => Square::Open,
                    '#' => Square::Tree,
                    _ => panic!("Input map is not valid.")
                })
                .collect())
        .collect();

    println!("There are {} trees on the way down the 3,1 slope.", count_trees(&map, 3, 1));

    let all_slopes_multiplied = count_trees(&map, 1, 1)
        * count_trees(&map, 3, 1)
        * count_trees(&map, 5, 1)
        * count_trees(&map, 7, 1)
        * count_trees(&map, 1, 2);
    println!("The product of all trees on all slopes is {}.", all_slopes_multiplied);
}

fn count_trees(map: &Vec<Vec<Square>>, move_right: usize, move_down: usize) -> u64 {
    map.iter()
        .enumerate()
        .filter(|(row_index, _)| row_index % move_down == 0 )
        .fold(0, |trees, (index, row)| {
            let column = (index * move_right) % row.len();
            trees + match row[column] {
                Square::Tree => 1,
                _ => 0,
            }
        })
}
