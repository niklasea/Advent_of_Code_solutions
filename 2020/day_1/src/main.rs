fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input: Vec<i64> = String::from_utf8(input).expect("Input file is not valid UTF8.")
        .lines()
        .map(|l| l.parse::<i64>().expect("Input must be a list of integers."))
        .collect();
    
    'outer: for (index, i) in input.iter().enumerate() {
        for j in &input[index+1..&input.len()-1] {
            if i + j == 2020 {
                println!("The two values {} and {} sum to 2020. Their product is: {}", i, j, i * j);
                break 'outer;
            }
        }
    }

    'outer: for (index, i) in input.iter().enumerate() {
        for j in &input[index+1..&input.len()-1] {
            for k in &input[index+2..&input.len()-1] {
                if i + j + k == 2020 {
                    println!("The three values {}, {} and {} sum to 2020. Their product is: {}", i, j, k, i * j * k);
                    break 'outer;
                }
            }
        }
    }
}
