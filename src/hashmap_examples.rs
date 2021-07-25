use std::collections::HashMap;

pub fn hashmap_example() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	let teams = vec![String::from("Blue"), String::from("Yellow")];

	let initial_scores = vec![10, 50];

	let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
	println!("{:?}", scores);

	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");

	let mut map = HashMap::new();
	map.insert(field_name, field_value);

	// 所有権がmapに移る
	// println!("{}", field_name);

	let blues_value = scores.get(&String::from("Blue"));
	match blues_value {
		Some(v) => println!("{}", v),
		_ => println!("unknown key"),
	}

	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	let text = "hello world wonderful world";
	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0); // keyがなければ0
		*count += 1;
	}

	println!("{:?}", map);
}
