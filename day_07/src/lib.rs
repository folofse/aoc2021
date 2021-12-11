use std::vec;

pub fn fuel(lines:Vec<String>, part1:bool)->i32 {
	let crabs:Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
	let mut total_fuel:i32 = 0;
	let mut least_fuel = 0;

	for c1 in 1..=*crabs.iter().max().unwrap() {
		total_fuel = 0;
		for c2 in crabs.clone() {
			let diff = i32::abs(c1 - c2);
			if part1 {
				total_fuel += diff;
			}else{
				let mut parent_diff = 0;
				for i in 1..=diff {
					parent_diff = i + parent_diff;
				}
				total_fuel += parent_diff;
			}
		}

		if total_fuel < least_fuel || least_fuel == 0 {
			least_fuel = total_fuel;
		}
	}
	return least_fuel;
}

#[cfg(test)]
mod tests {
	use crate::fuel;
	use shared::read_lines;

	#[test]
	#[ignore]
	fn fuel_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(fuel(lines, true), 37);
	}

	#[test]
	#[ignore]
	fn fuel_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(fuel(lines, true), 328187);
	}

	#[test]
	#[ignore]
	fn fuel_test3() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(fuel(lines, false), 168);
	}

	#[test]
	
	fn fuel_test4() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(fuel(lines, false), 168);
	}
	
}