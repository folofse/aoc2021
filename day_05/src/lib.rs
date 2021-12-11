extern crate regex;
use regex::Regex;
use std::cmp;

pub fn diagram(lines:Vec<String>, use_diagonal:bool)->i32 {
	
	let values:Vec<Vec<Vec<usize>>> = get_values(lines);
	let max_values = get_max_values(values.clone());
	
	let mut grid:Vec<Vec<i32>> = vec![vec![0; max_values[0]]; max_values[1]];

	values.iter().for_each(|row| {
		row.iter().for_each(|col| {
			if col[0] == col[2] { // Vertical
				for y in cmp::min(col[1], col[3])..=cmp::max(col[1], col[3]) {
					grid[y][col[0]] += 1;
				}
			}else if col[1] == col[3] { // Horizontal
				for x in cmp::min(col[0], col[2])..=cmp::max(col[0], col[2]) {
					grid[col[1]][x] += 1;
				}
			}

			if use_diagonal { // Diagonal

				let mut steps_x:Vec<usize> = (col[0]..=col[2]).collect::<Vec<_>>();
				let mut steps_y:Vec<usize> = (col[1]..=col[3]).collect::<Vec<_>>();

				if col[0] > col[2] {
					steps_x = (col[2]..=col[0]).collect::<Vec<_>>();
					steps_x.reverse();
				}
		
				if col[1] > col[3] {
					steps_y = (col[3]..=col[1]).collect::<Vec<_>>();
					steps_y.reverse();
				}

				if &steps_x.len() == &steps_y.len() {		
					for i in 0..steps_y.len() {
						grid[steps_y[i]][steps_x[i]] += 1;
					}
				}
			}
		});
	});


	let mut count = 0;
	for y in grid{
		for x in y {
			count = if x > 1 { count + 1 } else { count };
		}
	}

	return count;
}

pub fn get_values(lines:Vec<String>) -> Vec<Vec<Vec<usize>>>{
	let re:Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
	lines.iter().map(|row| 
		re.captures_iter(row).map(|capture| 
			vec![
				capture.get(1).unwrap().as_str().parse::<usize>().unwrap(),
				capture.get(2).unwrap().as_str().parse::<usize>().unwrap(),
				capture.get(3).unwrap().as_str().parse::<usize>().unwrap(),
				capture.get(4).unwrap().as_str().parse::<usize>().unwrap(),
			]
		).collect::<Vec<Vec<usize>>>()
	).collect()
}
pub fn get_max_values(values:Vec<Vec<Vec<usize>>>) -> Vec<usize> {
	let mut max_x:usize = 0;
	let mut max_y:usize = 0;

	values.iter().for_each(|row| {
		row.iter().for_each(|values| {
			max_x = if cmp::max(values[0], values[2]) > max_x { cmp::max(values[0], values[2]) } else { max_x };
			max_y = if cmp::max(values[1], values[3]) > max_y { cmp::max(values[1], values[3]) } else { max_y };
		});
	});

	vec![max_x+1, max_y+1]
}


#[cfg(test)]
mod tests {
	use crate::diagram;
	use shared::read_lines;

	#[test]
	#[ignore]
	fn diagram_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(diagram(lines, false), 5);
	}	

	#[test]
	#[ignore]
	fn diagram_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(diagram(lines, false), 5084);
	}	

	#[test]
	#[ignore]
	fn diagram_test3() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(diagram(lines, true), 12);
	}	
	
	#[test]
	#[ignore]
	fn diagram_test4() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(diagram(lines, true), 17882);
	}	
}