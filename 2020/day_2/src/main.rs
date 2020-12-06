use regex::Regex;

struct Policy {
    letter: char,
    min: usize,
    max: usize,
}

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input: Vec<(String, Policy)> = parse_input(String::from_utf8(input).expect("Input is not valid UTF-8."));

    let valid_passwords_old: Vec<String> = input
        .iter()
        .filter(|(password, policy)| valid_password_old(password.to_string(), policy))
        .map(|(password, _)| password.clone())
        .collect();
    
    println!("There are {} valid passwords in the list according to the old policy.", valid_passwords_old.len());

    let valid_passwords_new: Vec<String> = input
    .iter()
        .filter(|(password, policy)| valid_password_new(password.to_string(), policy))
        .map(|(password, _)| password.clone())
        .collect();
    
    println!("There are {} valid passwords in the list according to the new policy.", valid_passwords_new.len());
}

fn parse_input(input: String) -> Vec<(String, Policy)> {
    let re: Regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[[:alpha:]]): (?P<password>[[:alpha:]]+)").unwrap();
    re.captures_iter(&input)
        .map(|c| 
            (c["password"].to_string(),
            Policy {
                letter: c["letter"].parse::<char>().unwrap(),
                min: c["min"].parse::<usize>().unwrap(),
                max: c["max"].parse::<usize>().unwrap(),
            }))
        .collect()
}

fn valid_password_old(password: String, policy: &Policy) -> bool {
    let count: usize = password
        .matches(policy.letter)
        .collect::<Vec<&str>>()
        .len();
    count <= policy.max && count >= policy.min
}

fn valid_password_new(password: String, policy: &Policy) -> bool {
    let chars: Vec<char> = password.chars().collect();
    // Potential index out of range
    match (chars[policy.min - 1] == policy.letter, chars[policy.max - 1] == policy.letter) {
        (true, false) => true,
        (false, true) => true,
        _ => false
    }
}
