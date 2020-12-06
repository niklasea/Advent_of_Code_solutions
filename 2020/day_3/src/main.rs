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

    let mut trees = 0;
    for (index, row) in map.iter().enumerate() {
        let column = (index * 3) % row.len();
        if row[column] == Square::Tree {
            trees += 1;
        }
    }
    println!("There are {} trees on the way down the 3,1 slope.", trees);
}
