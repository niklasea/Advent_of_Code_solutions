fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input file is not valid UTF-8.");

    let (preamble, message): (Vec<(usize, i64)>, Vec<(usize, i64)>) = input.lines()
        .map(|x| x.parse().unwrap())
        .enumerate()
        .partition(|&(index, _)| index < 25);

    let message: Vec<i64> = message.iter()
        .cloned()
        .map(|(_, x)| x)
        .collect();

    let mut previous_25: Vec<i64> = preamble.iter().map(|(_, x)| *x).collect();
    let not_the_sum_of_prev = message.iter()
        .cloned()
        .find(|current|
            match find_summands(&previous_25, current) {
                Some(_) => {
                    previous_25.remove(0);
                    previous_25.push(*current);
                    false
                },
                None => true
            }
        )
        .unwrap();
    println!("The first number to not be the sum of two of the previous 25 numbers is {}.", not_the_sum_of_prev);

    let (smallest, largest) = message.iter()
            .enumerate()
            .find_map(|(begin, _)| {
                let i = (begin..(message.len() - 1))
                    .find_map(|i| {
                        match message[begin..i].iter().sum::<i64>() == not_the_sum_of_prev {
                            true => Some(i),
                            false => None,
                        }
                    });
                if let Some(end) = i {
                    Some(
                        (message[begin..end].iter().min().unwrap(),
                        message[begin..end].iter().max().unwrap())
                    )
                } else {
                    None
                }
            })
            .unwrap();
    println!("The encryption weakness is the sum {}, composed of {} and {}.", smallest + largest, smallest, largest);
}

fn find_summands(list: &Vec<i64>, sum: &i64) -> Option<(i64, i64)> {
    list.iter()
        .enumerate()
        .find_map(|(index, x)|
            match list[index + 1..].iter().find(|y| *y + x == *sum) {
                Some(other) => return Some((*x, *other)),
                None => None,
            }
        )
}
