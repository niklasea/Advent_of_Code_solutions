#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() {
	let input = std::fs::read("./input.txt").expect("Unable to read input file.");
	let input = String::from_utf8(input).expect("Input file is not valid UTF-8.");

	let bags: HashMap<String, Vec<String>> = input.lines()
		.map(|x| bag_from_string(x.to_string()))
		.collect();

	let shiny_gold = bags.keys()
		.filter(|x| contains_bag_of_color(
			&bags,
			x.to_string(),
			String::from("shiny gold")
		))
		.count();

	println!("There are {} bags directly or indirectly capable of containing shiny gold bags.", shiny_gold);
}

fn bag_from_string(input: String) -> (String, Vec<String>) {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"(\w+ \w+) bags?").unwrap();
	}
	let mut iter = RE.captures_iter(&input);
	
	(iter.next()
		.expect("Not a valid bag: empty line.")[1]
		.to_string(),
	iter.map(|x| x[1].to_string())
		.filter(|x| x != "no other")
		.collect())
}

fn contains_bag_of_color(bags: &HashMap<String, Vec<String>>, source: String, target: String) -> bool {
	if bags[&source].contains(&target) {
		true
	} else {
		bags[&source].iter()
			.any(|x| contains_bag_of_color(
				bags, 
				x.to_string(), 
				target.to_string()
			))
	}
}
