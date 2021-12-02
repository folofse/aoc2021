
pub fn movement(lines:Vec<String>)->i32 {
	let mut x:i32 = 0;
	let mut y:i32 = 0;

	for line in lines {
		let strSplit = line.split_whitespace().collect::<Vec<&str>>();
		let direction:&str = strSplit[0];
		let speed:i32 = strSplit[1].parse::<i32>().unwrap();

		match direction {
			"forward" => x += speed,
			"up" => y -= speed,
			"down" => y += speed,
			_ => println!("defualt?"),
		}
			
	}

	return x * y
}




#[cfg(test)]
mod tests {
	use crate::movement;
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

	fn movement_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(movement(lines), 1499229);
	}
   
}