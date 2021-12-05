
pub fn diagnose(lines:Vec<String>)->i32 {



	return i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap();
}

pub fn life_support(lines:Vec<String>)->i32 {

	let oxygen = find_value(&lines, "oxygen");
	let scrubber = find_value(&lines, "scrubber");

	return i32::from_str_radix(&oxygen, 2).unwrap() * i32::from_str_radix(&scrubber, 2).unwrap();
}




#[cfg(test)]
mod tests {
   
	#[test]
	
	fn diagnose_test1() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(diagnose(lines), 1);
	}	
}