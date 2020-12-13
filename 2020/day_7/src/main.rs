#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

struct Bag {
	count: u64,
	color: String,
}

fn main() {
	let input = std::fs::read("./input.txt").expect("Unable to read input file.");
	let input = String::from_utf8(input).expect("Input file is not valid UTF-8.");

	let bags: HashMap<String, Vec<Bag>> = input.lines()
		.map(|x| bag_from_string(x.to_string()))
		.collect();

	let shiny_gold_containers = bags.keys()
		.filter(|x| contains_bag_of_color(
			&bags,
			x.to_string(),
			String::from("shiny gold")
		))
		.count();

	println!("There are {} bags directly or indirectly capable of containing shiny gold bags.", shiny_gold_containers);

	let my_bag = "shiny gold";
	println!("My {} bag contains {} other bags.", my_bag, number_of_bags_contained(&bags, my_bag.to_string()));
}

fn bag_from_string(input: String) -> (String, Vec<Bag>) {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"(?P<count>\d*) ?(?P<color>\w+ \w+) bags?").unwrap();
	}
	let mut bag_matches = RE.captures_iter(&input);
	
	(bag_matches.next()
		.expect("Not a valid bag: empty line.")["color"]
		.to_string(),
	bag_matches.map(|b| Bag {
			count: b["count"].parse().unwrap_or_default(),
			color: b["color"].to_string()
		})
		.filter(|bag| bag.color != "no other")
		.collect())
}

fn contains_bag_of_color(bags: &HashMap<String, Vec<Bag>>, source: String, target: String) -> bool {
	if bags[&source].iter().any(|bag| bag.color == target) {
		true
	} else {
		bags[&source].iter()
			.any(|bag| contains_bag_of_color(
				bags, 
				bag.color.to_string(), 
				target.to_string()
			))
	}
}

fn number_of_bags_contained(bags: &HashMap<String, Vec<Bag>>, source: String) -> u64 {
	bags[&source].iter()
		.fold(0, |sum, bag| {
			sum + bag.count + bag.count * number_of_bags_contained(
				&bags,
				bag.color.to_string()
			)
		})
}
