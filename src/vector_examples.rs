pub fn vector_example() {
	let v1: Vec<i32> = Vec::new();
	let mut v2 = Vec::new();

	v2.push(1);
	v2.push(2);
	v2.push(3);
	v2.push(4);
	v2.push(5);

	let v = vec![1, 2, 3, 4, 5];

	// let does_not_exist = &v[100];
	// getだとOption型
	if let Some(n) = v.get(100) {
		println!("{}", n);
	}

	let mut mv = vec![1, 2, 3, 4, 5];
	let first = &mv[0];

	println!("The first element is: {}", first);

	for i in &mv {
		println!("{}", i);
	}

	for i in &mut mv {
		*i += 1;
	}

	println!("{:?}", mv);
}
