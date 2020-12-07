use regex::RegexSet;

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input file is not valid UTF-8.");
    let input = input.split("\n\n");
    
    let passports: Vec<&str> = input.filter(|p| has_required_fields(p.to_string())).collect();
    println!("There are {} passports with the required fields.", passports.len());
    
    let passports: Vec<&str> = passports.into_iter()
        .filter(|p| has_valid_fields(p.to_string()))
        .collect();
    println!("Of those passports, {} are valid.", passports.len());
}

fn has_required_fields(input: String) -> bool {
    let set = RegexSet::new(&[
        r"\bbyr:.+\b",
        r"\biyr:.+\b",
        r"\beyr:.+\b",
        r"\bhgt:.+\b",
        r"\bhcl:.+\b",
        r"\becl:.+\b",
        r"\bpid:.+\b",
    ]).unwrap();
    let matches: Vec<_> = set.matches(&input).into_iter().collect();
    matches == vec![0, 1, 2, 3, 4, 5, 6]
}


fn has_valid_fields(input: String) -> bool {
    // This is an abomination.
    let set = RegexSet::new(&[
        r"\bbyr:((19[2-9]\d)|(200[0-2]))\b",
        r"\biyr:(20((1\d)|(20)))\b",
        r"\beyr:(20((2\d)|(30)))\b",
        r"\bhgt:((1(([5-8]\d)|(9[0-3])))cm|((59)|(6\d)|(7[0-6]))in)\b",
        r"\bhcl:#[0-9a-f]{6}\b",
        r"\becl:((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth))\b",
        r"\bpid:\d{9}\b",
    ]).unwrap();
    let matches: Vec<_> = set.matches(&input).into_iter().collect();
    matches == vec![0, 1, 2, 3, 4, 5, 6]
}
