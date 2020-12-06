use regex::Regex;

struct Policy {
    letter: char,
    min: u64,
    max: u64,
}

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input: String = String::from_utf8(input).expect("Input is not valid UTF-8.");

    let valid_passwords: Vec<String> = parse_input(input)
        .into_iter()
        .filter(|(password, policy)| valid_password(password.to_string(), policy))
        .map(|(password, _)| password)
        .collect();
    
    println!("There are {} valid passwords in the list.", valid_passwords.len());
}

fn parse_input(input: String) -> Vec<(String, Policy)> {
    let re: Regex = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[[:alpha:]]): (?P<password>[[:alpha:]]+)").unwrap();
    re.captures_iter(&input)
        .map(|c| 
            (c["password"].to_string(),
            Policy {
                letter: c["letter"].parse::<char>().unwrap(),
                min: c["min"].parse::<u64>().unwrap(),
                max: c["max"].parse::<u64>().unwrap(),
            }))
        .collect()
}

fn valid_password(password: String, policy: &Policy) -> bool {
    let count: u64 = password
        .matches(policy.letter)
        .collect::<Vec<&str>>()
        .len() as u64;
    count <= policy.max && count >= policy.min
}
