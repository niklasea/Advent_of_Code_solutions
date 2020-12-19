use std::thread;

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input is not valid UTF-8.");

    let mut input: Vec<i64> = input.lines()
        .map(|x| x.parse().expect("Not a valid integer."))
        .collect();
    input.sort();
    input.insert(0, 0);

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
                0 => (),
                1 => one_joltages += 1,
                2 => (),
                3 => three_joltages += 1,
                _ => panic!("Not a valid chain of adapters."),
            }
        });

    println!("In this chain there are {} differences of 1 jolt and {} differences of 3 jolts.", one_joltages, three_joltages);

    thread::Builder::new().stack_size(1_000_000_000 as usize).spawn(move || {
        println!("There are {} different ways to arrange the adapters.", num_adapter_combinations(input));
    }).unwrap().join().ok();
}

fn num_adapter_combinations(adapters: Vec<i64>) -> i64 {
    let mut combinations = 1;
    let mut index = 0;
    while index < adapters.len() {
        let end_index = index + first_convergence(adapters[index..].to_vec());
        if end_index > index + 1 {
            combinations *= num_adapter_combinations_recursive(adapters[index..end_index].to_vec());
        }
        index = end_index;
    }
    combinations
}

fn first_convergence(adapters: Vec<i64>) -> usize {
    adapters[1..].iter()
        .cloned()
        .enumerate()
        .find_map(|(index, output)| {
            if output - adapters[index] == 3 {
                Some(index + 1)
            } else {
                None
            }
        })
        .unwrap_or_else(|| adapters.len())
}

fn num_adapter_combinations_recursive(adapters: Vec<i64>) -> i64 {
    match adapters.len() {
        1 => 1,
        _ => {
            adapters[1..].iter()
                .cloned()
                .enumerate()
                .filter(|(_, output)| (output - adapters[0]) <= 3)
                .map(|(index, _)| num_adapter_combinations_recursive(adapters[index + 1..].to_vec()))
                .sum::<i64>()
        },
    }
}
