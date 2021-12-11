use std::vec;



pub fn fish(lines:Vec<String>, max_days:i32)->i64 {

	let fishes:Vec<usize> = lines[0].split(",").map(|age| age.parse::<usize>().unwrap()).collect();
	let mut fish_ages = vec![0; 9];

	for age in fishes{
		fish_ages[age] += 1;
	}

	for day in 0..max_days {
		
		let mut counter = vec![0; 9]; 
		for i in 0..fish_ages.len() {
			if i != 0 {
				counter[i-1] = fish_ages[i];
			}

		}
		counter[8] = fish_ages[0]; // New fishes
		counter[6] = counter[6] + fish_ages[0]; // New cycle, add previous cycle

		fish_ages = counter.clone();
		println!("{:?}", counter);
		
		
	}
	println!("{:?}", fish_ages);
	return fish_ages.iter().sum();
}


#[cfg(test)]
mod tests {
	use crate::fish;
	use shared::read_lines;

	#[test]
	#[ignore]
	fn fish_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(fish(lines, 80), 5934);
	}

	#[test]
	#[ignore]
	fn fish_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(fish(lines, 80), 371379);
	}	

	#[test]
	#[ignore]
	fn fish_test3() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(fish(lines, 256), 26984457539);
	}

	#[test]
	fn fish_test4() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(fish(lines, 256), 1674303997472);
	}	
}