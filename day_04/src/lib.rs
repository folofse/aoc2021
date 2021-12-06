pub fn bingo(mut lines:Vec<String>)->i32 {
	
	let call_out_numbers:Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
	lines.remove(0);

	let (mut boards, mut boards_total) = parse_boards(lines);

	let mut called_num = -1;
	let mut winning_total = -1;

	'outer: for num in call_out_numbers {
		called_num = num;

		for i in 0..boards.len() {
			let mut total = boards_total[i];
			let board = boards[i].clone();

			for j in 0..board.len() {
				let row = &board[j];

				for k in 0..row.len() {
					if row[k] == called_num {
						boards[i][j].remove(k);
						total -= called_num;
					}
				}
				
				if boards[i][j].len() == 0 {
					println!("{}", total);
					winning_total = total;
					break 'outer;
				}
			}

			boards_total[i] = total;
		}
	}

	return called_num * winning_total;
}

fn parse_boards(lines:Vec<String>) -> (Vec<Vec<Vec<i32>>>, Vec<i32>)  {
	let mut boards:Vec<Vec<Vec<i32>>> = vec![];
	let mut board:Vec<Vec<i32>> = vec![];

	let mut boards_total:Vec<i32> = vec![];
	let mut board_total:i32 = 0;
	let mut row_count = 0;

	for line in lines {
		if line == "" {
			row_count = 0;
			continue;
		}

		let row = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>().clone();
		board_total += row.iter().sum::<i32>();
		board.push(row);
		
		row_count += 1;

		if row_count == 5 {
			boards_total.push(board_total);
			boards.push(board);
			
			//Reset
			board = vec![];
			board_total = 0;
		}
	}

	return (boards, boards_total);
}

#[cfg(test)]
mod tests {
	use crate::bingo;
	use shared::read_lines;

	#[test]
	#[ignore]
	fn bingo_test1() {
		let lines = read_lines("./src/example.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(bingo(lines), 4512);
	}	

	#[test]
	fn bingo_test2() {
		let lines = read_lines("./src/input.txt")
			.iter()
			.map(|f| f.parse().unwrap())
			.collect();

		assert_eq!(bingo(lines), 65325);
	}	
}