use std::collections::HashSet;

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input is not valid UTF-8.");

    let groups: Vec<HashSet<char>> = input.split("\n\n")
        .map(|x| x.lines()
            .flat_map(|l| l.chars())
            .collect())
        .collect();

    let sum_of_yes = groups.iter()
        .fold(0, |sum, group| sum + group.len());
    println!("The sum of \"yes\" answers from all groups on the plane is: {}", sum_of_yes);
}
