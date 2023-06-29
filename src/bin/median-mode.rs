use std::collections::HashMap;
use std::io;

fn main() {
	// Ask the user for input
	println!("Input a sorted list of integers seperated by spaces: ");
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");

	// Parse input
	let input = input.trim();
	let mut ints: Vec<i32> = Vec::new();
	for int in input.split(' ') {
		ints.push(int.parse().unwrap());
	}
	
	// Find the median
	let mut median_index = ints.len();
	median_index = (if median_index % 2 == 0 { median_index / 2 } else { ((median_index - 1) / 2) + 1 }) - 1;
	println!("The median value is {}", ints[median_index]);

	// Find the mode
	let mut counts: HashMap<i32, u64> = HashMap::new();
	for int in ints {
		let count = counts.entry(int).or_insert(0);
		*count += 1;
	}
	let mut highest_int = 0;
	let mut highest_count: u64 = 0;
	for (int, count) in counts {
		if count > highest_count {
			highest_int = int;
			highest_count = count;
		}
	}
	println!("The mode value is {highest_int}");
}
