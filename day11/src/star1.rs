use std::collections::HashSet;

pub fn star1(input: &str) -> i32 {
    let raw_gid = input.lines().map(|l| l.chars().collect::<Vec<_>>());
    let mut grid = vec![];
    for chars in raw_gid {
        grid.push(chars.clone());
        if chars.iter().all(|c| c == &'.') {
            grid.push(chars.clone())
        }
    }
    let transposed = (0..grid[0].len())
        .map(|col| (0..grid.len()).map(|row| grid[row][col]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut expanded_grid = vec![];
    for chars in transposed.clone() {
        expanded_grid.push(chars.clone());
        if chars.iter().all(|c| c == &'.') {
            expanded_grid.push(chars.clone())
        }
    }
    let mut set:HashSet<(i32, i32)> = HashSet::new();
    expanded_grid.iter().enumerate().for_each(|(i, chars)| chars.iter().enumerate().for_each(|(j, c)| if c == &'#' {set.insert((i.try_into().unwrap(), j.try_into().unwrap()));}));
    let mut distance = 0;
    for from in set.clone() {
        set.remove(&from);
        for to in set.clone() {
            distance += (from.0 - to.0).abs() + (from.1 - to.1).abs();
        }
    }
    distance
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 374)
    }
}
