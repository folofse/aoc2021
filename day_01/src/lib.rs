pub fn count(lines:Vec<i32>)->i32 {
    let mut sum = 0;
    let mut parent:i32 = lines[0];

    for line in lines {
        if line > parent {
            sum += 1;
        }
        parent = line
    }

    return sum
}

pub fn window(lines:Vec<i32>)->i32 {
    let mut sum = 0;
    let mut parent:i32 = lines[0] + lines[1] + lines[2];
    let mut window:i32;
    
   for i in 1..lines.len() {
    if i + 2 < lines.len() {
      window = lines[i] + lines[i+1] + lines[i+2];
      if window > parent {
        sum += 1;
      }
      parent = window
    }
  }

    return sum
}


#[cfg(test)]
mod tests {
    use crate::count;
    use crate::window;
    use shared::read_lines;

    #[test]
    #[ignore]
    fn count_test() {
        let lines = read_lines("./src/example.txt")
            .iter()
            .map(|f| f.parse().unwrap())
            .collect();

        assert_eq!(count(lines), 7);
    }

    #[test]
    #[ignore]
    fn count_test_2() {
        let lines = read_lines("./src/input.txt")
            .iter()
            .map(|f| f.parse().unwrap())
            .collect();

        assert_eq!(count(lines), 1696);
    }

    #[test]
    #[ignore]
    fn window_test() {
        let lines = read_lines("./src/example.txt")
            .iter()
            .map(|f| f.parse().unwrap())
            .collect();

        assert_eq!(window(lines), 5);
    }

    #[test]
    fn window_test_2() {
        let lines = read_lines("./src/input.txt")
            .iter()
            .map(|f| f.parse().unwrap())
            .collect();

        assert_eq!(window(lines), 1737);
    }
}