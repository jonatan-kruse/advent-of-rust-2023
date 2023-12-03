pub fn star1(input: &str) -> u32 {
    let char_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let numbers = find_numbers(char_grid);
    return numbers.iter().sum();
}

fn find_numbers(grid: Vec<Vec<char>>) -> Vec<u32> {
    let g = &grid;
    g.iter()
        .enumerate()
        .map(|(row_num, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_num, _)| get_number(row_num, col_num, g))
        })
        .flatten()
        .collect()
}

fn get_number(row: usize, col: usize, grid: &Vec<Vec<char>>) -> Option<u32> {
    let c = grid[row][col];
    if (col > 0 && grid[row][col - 1].is_digit(10) && c.is_digit(10)) || !c.is_digit(10) {
        return None;
    }

    let mut number_length: usize = 1;
    while col + number_length < grid[0].len() && grid[row][col + number_length].is_digit(10) {
        number_length += 1
    }

    let mut num = 0;
    for i in 0..number_length {
        num += grid[row][col + i].to_digit(10).unwrap()
            * ((10 as usize).pow((number_length - i - 1) as u32)) as u32;
    }

    if is_next_to_symbol(row, col, grid.clone(), number_length) {
        return Some(num);
    }
    return None;
}

fn is_next_to_symbol(row: usize, col: usize, grid: Vec<Vec<char>>, number_length: usize) -> bool {
    let mut cols: Vec<usize> = vec![];
    let mut rows: Vec<usize> = vec![];
    for i in 0..3 {
        let check: i32 = (row + i) as i32 - 1;
        if check > 0 && check < (grid.len() - 1).try_into().unwrap() {
            rows.push(check.try_into().unwrap())
        }
    }

    for i in 0..(number_length + 2) {
        let check: i32 = (col + i) as i32 - 1;
        if check > 0 && check < (grid[0].len() - 1).try_into().unwrap() {
            cols.push(check.try_into().unwrap())
        }
    }

    for row_test in rows {
        for col_test in cols.clone() {
            if is_symbol(grid[row_test][col_test]) {
                return true;
            }
        }
    }
    false
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 4361)
    }
}
