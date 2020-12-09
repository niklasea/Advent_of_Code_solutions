use std::collections::HashSet;

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input is not valid UTF-8.");

    let groups: Vec<Vec<HashSet<char>>> = input.split("\n\n")
        .map(|x| x.lines()
            .map(|l| l.chars().collect())
            .collect())
        .collect();

    let union_of_answers = groups.iter()
        .fold(0, |total, group| { 
            total
            + group.iter()
                .flatten()
                .collect::<HashSet<_>>()
                .len()
        });
    println!("The union of \"yes\" answers from all groups on the plane is: {}", union_of_answers);

    let start_set: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let intersection_of_answers = groups.iter()
        .fold(0, |total, group| 
            total
            + group.iter()
                .fold(start_set.clone(), |answers, person| {
                    answers.intersection(&person)
                        .cloned()
                        .collect()
                })
                .len()
        );
    println!("The intersection of \"yes\" answers from all groups on the plane is: {}", intersection_of_answers);
}   
