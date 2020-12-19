fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input is not valid UTF-8.");

    let mut input: Vec<i64> = input.lines()
        .map(|x| x.parse().expect("Not a valid integer."))
        .collect();
    input.sort();

    let mut one_joltages = 0;
    let mut three_joltages = 1;
    input.iter()
        .enumerate()
        .for_each(|(index, output)| {
            let last = if index > 0 {
                input[index-1]
            } else {
                0
            };
            let difference = output - last;
            match difference {
                1 => one_joltages += 1,
                2 => (),
                3 => three_joltages += 1,
                _ => panic!("Not a valid chain of adapters."),
            }
        });

    println!("In this chain there are {} differences of 1 jolt and {} differences of 3 jolts.", one_joltages, three_joltages);
}
