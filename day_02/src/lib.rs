
pub fn movement(lines:Vec<String>)->i32 {
	let mut x:i32 = 0;
	let mut y:i32 = 0;

	for line in lines {
		let str_split = line.split_whitespace().collect::<Vec<&str>>();
		let direction:&str = str_split[0];
		let speed:i32 = str_split[1].parse::<i32>().unwrap();

		match direction {
			"forward" => x += speed,
			"up" => y -= speed,
			"down" => y += speed,
			_ => println!("default?"),
		}
	}

	return x * y
}

pub fn aim(lines:Vec<String>)->i32 {
	let mut x:i32 = 0;
	let mut y:i32 = 0;
	let mut aim:i32 = 0;

	for line in lines {
		let str_split = line.split_whitespace().collect::<Vec<&str>>();
		let direction:&str = str_split[0];
		let speed:i32 = str_split[1].parse::<i32>().unwrap();

		match direction {
			"forward" => {
				x += speed;
				y += aim * speed;
			},
			"up" => aim -= speed,
			"down" => aim += speed,
			_ => println!("defualt?"),
		}
	}

	return x * y
}


#[cfg(test)]
mod tests {
	use crate::movement;
	use crate::aim;
	use shared::read_lines;

	#[test]
	#[ignore]
	fn movement_test() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(movement(lines), 150);
	}

	#[test]
	#[ignore]
	fn movement_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(movement(lines), 1499229);
	}

	#[test]
	#[ignore]
	fn aim_test() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(aim(lines), 900);
	}

	#[test]
	#[ignore]
	fn aim_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(aim(lines), 1340836560);
	}
   
}