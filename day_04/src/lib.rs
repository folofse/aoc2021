use std::collections::HashMap;

pub fn bingo1(mut lines:Vec<String>)->i32 {
	
	let call_out_numbers:Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
	lines.remove(0);

	let boards = parse_boards(lines);

	return play(call_out_numbers, boards, true);
}

pub fn bingo2(mut lines:Vec<String>)->i32 {
	
	let call_out_numbers:Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
	lines.remove(0);

	let boards = parse_boards(lines);

	return play(call_out_numbers, boards, false);
}

fn play(call_out_numbers:Vec<i32>, mut boards:Vec<Vec<Vec<i32>>>, find_first:bool)->i32 {
	let mut final_num = -1;
	let mut final_sum = 0;
	let mut board_won:HashMap<usize, usize> = HashMap::new();
	let mut winning_board= vec![];



	'call_outs: for num in call_out_numbers {
		'boards: for i in 0..boards.len() {
			'row_cols: for j in 0..boards[i].len() {

				boards[i][j].retain(|x| *x != num);

				if boards[i][j].len() == 0 {
					
					final_num = num;
					
					if find_first {
						winning_board = boards[i].clone();
						break 'call_outs;
					}else{
						board_won.insert(i, i);
						
						if board_won.len() == boards.len() {
							winning_board = boards[i].clone();
							break 'call_outs;
						}
					}
				}
			}
		}
	}	

	for i in 0..winning_board.len()/2 {
		final_sum += winning_board[i].iter().sum::<i32>();
	}
	
	return final_num * final_sum;
}

fn parse_boards(lines:Vec<String>) -> Vec<Vec<Vec<i32>>>  {
	let mut boards:Vec<Vec<Vec<i32>>> = vec![];
	let mut board:Vec<Vec<i32>> = vec![];

	let mut row_count = 0;

	for line in lines {
		if line == "" {
			row_count = 0;
			continue;
		}

		// Add rows
		let row = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>().clone();
		board.push(row);
		
		row_count += 1;

		if row_count == 5 {

			// Add cols as well
			for i in 0..board.clone().len() {
				let mut col:Vec<i32> = vec![];	
				for j in 0..board[i].len() {
					col.push(board[j][i]);
				}
				board.push(col);
			}

			boards.push(board);
			
			// Reset
			board = vec![];
		}
	}

	return boards;
}

#[cfg(test)]
mod tests {
	use crate::bingo1;
	use crate::bingo2;
	use shared::read_lines;

	#[test]
	#[ignore]
	fn bingo1_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(bingo1(lines), 4512);
	}	

	#[test]
	#[ignore]
	fn bingo1_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(bingo1(lines), 65325);
	}	


	#[test]
	#[ignore]
	fn bingo2_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(bingo2(lines), 1924);
	}

	#[test]
	
	fn bingo2_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(bingo2(lines), 4624);
	}	
}