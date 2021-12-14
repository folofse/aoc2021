use std::collections::HashMap;

pub fn part_1(lines:Vec<String>)->i32 {
	let patterns:Vec<&str> = vec![
		"cf",
		"bcdf",
		"acf",
		"abcdefg",
	];
	let mut occurrences: i32 = 0;

	for line in lines.iter() {
		let segments:Vec<&str> = line.split("|").map(|x| x).collect();
		// let signal_patterns:Vec<&str> = segments[0].split(" ").map(|x| x).collect();
		let four_digits:Vec<&str> = segments[1].split(" ").map(|x| x).collect();

		for fd in four_digits {
			for pattern in &patterns {
				if pattern.len() == fd.len(){
					occurrences += 1;
				}
			}
		}
	}

	return occurrences;
}

pub fn part_2(lines:Vec<String>)->i32{

	let mut sum = 0;

	for line in lines.iter() {
		let segments:Vec<&str> = line.split("|").map(|x| x).collect();
		let signal_patterns:Vec<&str> = segments[0].split(" ").map(|x| x).collect();
		let four_digits:Vec<&str> = segments[1].split(" ").map(|x| x).collect();
		let found_numbers = translate_pattern(signal_patterns);

		let mut sum_str = String::from("");

		for fd in four_digits {
			if fd == "" {
				continue;
			}
			for (num, pattern) in &found_numbers {
				if sort_string(pattern.clone()) == sort_string(String::from(fd)) {
					sum_str.push_str(num);
				}
			}
		}
		
		sum += sum_str.parse::<i32>().unwrap();
	}
	return sum
}
fn translate_pattern(numbers:Vec<&str>) -> HashMap<String, String> {
	
	// Known numbers
	let mut found_numbers:HashMap<String, String> = HashMap::new();
	
	numbers.iter().for_each(|num| {
		 match num.len() {
			2 => { // One
				found_numbers.insert(String::from("1"), String::from(*num));
			},
			3 => { // Seven
				found_numbers.insert(String::from("7"), String::from(*num));
			},
			4 => { // Four
				found_numbers.insert(String::from("4"), String::from(*num));
			},
			7 => { // Eight
				found_numbers.insert(String::from("8"), String::from(*num));
			},
			_ => { // 0, 2, 3, 5, 6, 9
				//println!("{}", num);
			} 
    }
	});

	// Run until all numbers are found
	while found_numbers.len() < numbers.len()-1 {
		found_numbers = find_numbers(numbers.clone(), found_numbers);
	}
	
	return found_numbers;
}

fn find_numbers<'a>(numbers:Vec<&str>, mut found_numbers:HashMap<String, String>) -> HashMap<String, String> {
	numbers.iter().for_each(|num| {
		
		let zero = 	if found_numbers.contains_key("0") {	found_numbers.get("0").unwrap().clone()	} else {	String::from("")	};
		let one = 	if found_numbers.contains_key("1") {	found_numbers.get("1").unwrap().clone()	} else {	String::from("")	};
		let two = 	if found_numbers.contains_key("2") {	found_numbers.get("2").unwrap().clone()	} else {	String::from("")	};
		let three = if found_numbers.contains_key("3") {	found_numbers.get("3").unwrap().clone()	} else {	String::from("")	};
		let four = 	if found_numbers.contains_key("4") {	found_numbers.get("4").unwrap().clone()	} else {	String::from("")	};
		let five = 	if found_numbers.contains_key("5") {	found_numbers.get("5").unwrap().clone()	} else {	String::from("")	};
		let six = 	if found_numbers.contains_key("6") {	found_numbers.get("6").unwrap().clone()	} else {	String::from("")	};
		let seven = if found_numbers.contains_key("7") {	found_numbers.get("7").unwrap().clone()	} else {	String::from("")	};
		// let eight = if found_numbers.contains_key("8") {	found_numbers.get("8").unwrap().clone()	} else {	String::from("")	};
		let nine = 	if found_numbers.contains_key("9") {	found_numbers.get("9").unwrap().clone()	} else {	String::from("")	};
		

		match num.len() {
			5 => { // 2 3 5

				if !four.is_empty() { //Find 2
					let four_diff = num.chars().fold(0, |acc, chr| if !four.contains(chr) { acc + 1 } else { acc }); 
					if four_diff == 3 && two.is_empty() {
						found_numbers.insert(String::from("2"), String::from(*num));
					}
				}	

				if !one.is_empty() { //Find 3
					let one_diff = num.chars().fold(0, |acc, chr| if !one.contains(chr) { acc + 1 } else { acc });
					if one_diff == 3 && three.is_empty() {
						found_numbers.insert(String::from("3"), String::from(*num));
					}
				}

				if !seven.is_empty() { //Find 3
					let seven_diff = num.chars().fold(0, |acc, chr| if !seven.contains(chr) { acc + 1 } else { acc }); 
					if seven_diff == 2 && three.is_empty() {
						found_numbers.insert(String::from("3"), String::from(*num));
					}
				}

				if !nine.is_empty() { //Find 2
					let nine_diff = num.chars().fold(0, |acc, chr| if !nine.contains(chr) { acc + 1 } else { acc }); 
					if nine_diff == 1 && two.is_empty() {
						found_numbers.insert(String::from("2"), String::from(*num));
					}
				}

				if !six.is_empty() { //Find 5
					let six_diff = num.chars().fold(0, |acc, chr| if !six.contains(chr) { acc + 1 } else { acc }); 
					if six_diff == 0 && five.is_empty() {
						found_numbers.insert(String::from("5"), String::from(*num));
					}
				}
			
				if !two.is_empty() && !three.is_empty(){ //Find 5
					let two_diff = num.chars().fold(0, |acc, chr| if !two.contains(chr) { acc + 1 } else { acc });
					let three_diff = num.chars().fold(0, |acc, chr| if !three.contains(chr) { acc + 1 } else { acc });

					if two_diff != 0 && three_diff != 0 && five.is_empty() {
						found_numbers.insert(String::from("5"), String::from(*num));
					}
				}
			}
			6 => { // 6 9 0
			
				if !one.is_empty() { //Find 6
					let one_diff = num.chars().fold(0, |acc, chr| if !one.contains(chr) { acc + 1 } else { acc }); 
					if one_diff == 5 && six.is_empty() {
						found_numbers.insert(String::from("6"), String::from(*num));
					}
				}

				if !three.is_empty() { //Find 6, 9
					let three_diff = num.chars().fold(0, |acc, chr| if !three.contains(chr) { acc + 1 } else { acc }); 
					if three_diff == 1 && nine.is_empty() {
						found_numbers.insert(String::from("9"), String::from(*num));
					}
				}	

				if !four.is_empty() { //Find 9
					let four_diff = num.chars().fold(0, |acc, chr| if !four.contains(chr) { acc + 1 } else { acc }); 
					if four_diff == 2 && nine.is_empty() {
						found_numbers.insert(String::from("9"), String::from(*num));
					}
				}	

				if !seven.is_empty() { //Find 6
					let seven_diff = num.chars().fold(0, |acc, chr| if !seven.contains(chr) { acc + 1 } else { acc });
					if seven_diff == 4 && six.is_empty() {
						found_numbers.insert(String::from("6"), String::from(*num));
					}
				}

				if !nine.is_empty() && !six.is_empty() { //Find 0
					let six_diff = num.chars().fold(0, |acc, chr| if !six.contains(chr) { acc + 1 } else { acc });
					let nine_diff = num.chars().fold(0, |acc, chr| if !nine.contains(chr) { acc + 1 } else { acc });
					if nine_diff != 0 && six_diff != 0 && zero.is_empty() {
						found_numbers.insert(String::from("0"), String::from(*num));
					}
				}
			
			}
			_ => {

			}
		}
	});

	return found_numbers;

}
fn sort_string(string:String) -> String {
	let mut chars:Vec<char> = string.chars().collect();
	chars.sort();
	return chars.iter().map(|c|c.to_string()).collect();
}


#[cfg(test)]
mod tests {
	use crate::part_1;
	use crate::part_2;
	use shared::read_lines;

	#[test]
	#[ignore]
	fn part1_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(part_1(lines), 26);
	}

	#[test]
	#[ignore]
	fn part1_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(part_1(lines), 554);
	}

	#[test]
	#[ignore]
	fn part2_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(part_2(lines), 61229);
	}

	#[test]
	
	fn part2_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(part_2(lines), 990964);
	}
	
}