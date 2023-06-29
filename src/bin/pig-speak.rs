use std::io;

fn main() {
	// Ask the user for input
	//println!("Enter an English sentence: ");
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
	let input = input.trim();
	
	// Translate to pig latin
	let vowels = ['a', 'e', 'i', 'o', 'u'];
	/*let mut words = Vec::from(input.split(' '));
	for word in words {
		if word.starts_with(&vowels) {
			word.push_str("hay");
		} else {
			word.push_str(format!("{}ay", word.remove()));
		}
	}
	
	// Output the translation
	let output = words.join(" ");
	println!("{output}");*/
	for word in input.split(' ') {
		let mut pig_word = String::from(word);
		if word.starts_with(&vowels) {
			pig_word.push_str("hay");
		} else {
			let first_char = pig_word.remove(0);
			pig_word.push_str(&format!("{}ay", first_char));
		}
		print!("{pig_word} ");
	}
	println!("");
}
