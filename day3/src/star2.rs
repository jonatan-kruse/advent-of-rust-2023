pub fn star2(input: &str) -> u32 {
    let char_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let numbers = find_gears(char_grid);
    return numbers.iter().sum();
}

#[derive(Clone)]
struct Star {
    numbers: u32,
    gear_ratio: u32,
}

fn find_gears(grid: Vec<Vec<char>>) -> Vec<u32> {
    let g = grid.clone();
    let mut star_grid: Vec<Vec<Star>> = vec![vec![Star { numbers: 0, gear_ratio: 1 }; grid[0].len()]; grid.len()];
    g.iter().enumerate().for_each(|(row_num, row)| {
        row.iter().enumerate().for_each(|(col_num, c)| {
            if col_num > 0 && grid[row_num][col_num - 1].is_digit(10) && c.is_digit(10) {
                return;
            }
            if c.is_digit(10) {
                let mut number_length: usize = 1;
                while col_num + number_length < grid[0].len()
                    && grid[row_num][col_num + number_length].is_digit(10)
                {
                    number_length += 1
                }
                let mut num = 0;
                for i in 0..number_length {
                    num += grid[row_num][col_num + i].to_digit(10).unwrap()
                        * ((10 as usize).pow((number_length - i - 1) as u32)) as u32;
                }
                let star_cords = get_neighboring_stars(row_num, col_num, grid.clone(), number_length);
                for (star_row, star_col) in star_cords {
                   let mut star = star_grid[star_row][star_col].clone();
                   star.numbers += 1;
                   star.gear_ratio *= num;
                   star_grid[star_row][star_col] = star
                }
            }
        });
    });
    let stars = star_grid.iter().flatten();
    return stars.filter(|s| s.numbers == 2).map(|s| s.gear_ratio).collect()
}

fn get_neighboring_stars(row: usize, col: usize, grid: Vec<Vec<char>>, number_length: usize) -> Vec<(usize, usize)> {
    let mut rows: Vec<usize> = vec![];
    let mut cols: Vec<usize> = vec![];

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

    let mut stars = vec![];
    for row_test in rows {
        for col_test in cols.clone() {
            if grid[row_test][col_test] == '*' {
                stars.push((row_test, col_test));
            }
        }
    }
    return stars;
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 467835)
    }
}
