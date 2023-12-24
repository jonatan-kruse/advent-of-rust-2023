pub fn star2(input: &str) -> usize {
    let mut grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut next_grid = cycle(&grid);
    let mut visited = Vec::new();
    while visited.iter().find(|g| **g == next_grid).is_none() {
        visited.push(next_grid.clone());
        next_grid = cycle(&next_grid);
    }
    let time_to_loop = visited.iter().position(|g| *g == next_grid).unwrap();
    let loop_length = visited.len() - time_to_loop;
    let remaining_cycles = (1000000000 - time_to_loop) % loop_length;
    grid = visited[time_to_loop + remaining_cycles - 1].clone();
    grid = rotate_clockwise(&grid);
    grid.iter().map(|col| calculate_weight(&col)).sum()
}

fn cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();
    for _ in 0..4 {
        new_grid = rotate_clockwise(&new_grid);
        new_grid = new_grid.iter().map(move_stones_right).collect();
    }
    new_grid
}

fn rotate_clockwise(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut new_grid = vec![vec![' '; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            new_grid[j][rows - 1 - i] = grid[i][j];
        }
    }
    new_grid
}

fn move_stones_right(col: &Vec<char>) -> Vec<char> {
    let mut new_col = col.clone();
    new_col.reverse();
    let mut i = 0;
    while i < new_col.len() {
        if new_col[i] == '.' {
           switch_stones(&mut new_col, i);
        }
        i += 1;
    }
    new_col.reverse();
    new_col
}

fn calculate_weight(col: &Vec<char>) -> usize {
    col.iter().enumerate().map(|(i, c)| if *c == 'O' { i + 1 } else { 0 }).sum()
}

fn switch_stones(col: &mut Vec<char>, i: usize) {
    let mut j = i;
    while j < col.len() && col[j] == '.' {
        j += 1;
    }
    if j < col.len() && col[j] == 'O'{
        col[i] = col[j];
        col[j] = '.';
    }
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 64)
    }
}

