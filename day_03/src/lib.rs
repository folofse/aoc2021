
pub fn diagnose(lines:Vec<String>)->i32 {
	let char_len = lines[0].split("").collect::<Vec<&str>>().len()-2;
	let mut ones = vec![0; char_len];

	for line in &lines {
		let str_split = line.split("").collect::<Vec<&str>>();

		for i in 0..char_len{
			if str_split[i+1] == "1"{
				ones[i] += 1;
			}
		}
	}

	let mut gamma = "".to_string();
	let mut epsilon = "".to_string();
	for i in 0..char_len{
		if ones[i] > lines.len()/2 {
			gamma.push_str("1");
			epsilon.push_str("0");
		} else {
			gamma.push_str("0");
			epsilon.push_str("1");
		}
	}


	return i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap();
}

pub fn life_support(lines:Vec<String>)->i32 {

	let oxygen = find_value(&lines, "oxygen");
	let scrubber = find_value(&lines, "scrubber");

	return i32::from_str_radix(&oxygen, 2).unwrap() * i32::from_str_radix(&scrubber, 2).unwrap();
}

pub fn find_value<'a>(lines:&'a Vec<String>, value_type:&'a str)->String {
	let char_len = lines[0].split("").collect::<Vec<&str>>().len()-2;
	let mut list = lines.clone();
	let mut ones = vec![];
	let mut zeroes = vec![];


	for col in 0..char_len{
		for row in 0..list.len() {
		
			let row_split = &list[row].split("").collect::<Vec<&str>>();

			if row_split[col+1] == "1" {
				ones.push(list[row].clone());
			}else{
				zeroes.push(list[row].clone())
			}

		}

		if ones.len() == zeroes.len() {
				if value_type == "oxygen" {
					list = ones.clone();
				}else{
					list = zeroes.clone();
				}
			} else if ones.len() > zeroes.len() {
				if value_type == "oxygen" {
					list = ones.clone();
				}else{
					list = zeroes.clone();
				}
			}else{
				if value_type == "oxygen" {
					list = zeroes.clone();
				}else{
					list = ones.clone();
				}
			}

			if list.len() == 1 {
				break;
			}
			ones.clear();
			zeroes.clear();
	}	
		
	return list[0].clone();
}




#[cfg(test)]
mod tests {
	use crate::diagnose;
	use crate::life_support;
	use shared::read_lines;

	#[test]
	#[ignore]
	fn diagnose_test() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(diagnose(lines), 198);
	}

	#[test]
	#[ignore]
	fn diagnose_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(diagnose(lines), 3813416);
	}

	#[test]
	#[ignore]
	fn life_support_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(life_support(lines), 230);
	}	
   
	#[test]
	#[ignore]
	fn life_support_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(life_support(lines), 2990784);
	}	
}