fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input file is not valid UTF-8.");

    let (preamble, message): (Vec<(usize, i64)>, Vec<(usize, i64)>) = input.lines()
        .map(|x| x.parse().unwrap())
        .enumerate()
        .partition(|&(index, _)| index < 25);

    let mut previous_25: Vec<i64> = preamble.iter().map(|(_, x)| *x).collect();
    let not_the_sum_of_prev = message.iter()
        .map(|(_, x)| *x)
        .find(|current| {
            match find_summands(&previous_25, *current) {
                Some(_) => {
                    previous_25.remove(0);
                    previous_25.push(*current);
                    false
                },
                None => true
            }
        });
    if let Some(number) = not_the_sum_of_prev {
        println!("The first number to not be the sum of two of the previous 25 numbers is {}.", number);
    }
}

fn find_summands(list: &Vec<i64>, sum: i64) -> Option<(i64, i64)> {
    list.iter()
        .enumerate()
        .find_map(|(index, x)|
            match list[index + 1..].iter().find(|y| *y + x == sum) {
                Some(other) => return Some((*x, *other)),
                None => None,
            }
        )
}
