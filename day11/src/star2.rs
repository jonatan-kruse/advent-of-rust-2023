use std::collections::HashSet;

pub fn star2(input: &str) -> i64 {
    let raw_gid = input.lines().map(|l| l.chars().collect::<Vec<_>>());
    let mut grid = vec![];
    for chars in raw_gid {
        if chars.iter().all(|c| c == &'.') {
            grid.push((0..chars.len()).map(|_| '▒').collect());
        } else {
            grid.push(chars.clone());
        }
    }
    let transposed = (0..grid[0].len())
        .map(|col| {
            (0..grid.len())
                .map(|row| grid[row][col])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut expanded_grid = vec![];
    for chars in transposed.clone() {
        if chars.iter().all(|c| c == &'.' || c == &'▒') {
            expanded_grid.push((0..chars.len()).map(|_| '▒').collect());
        } else {
            expanded_grid.push(chars.clone());
        }
    }
    let mut set: HashSet<(i64, i64)> = HashSet::new();
    let mut x_expand = 0;
    let mut y_expand = 0;
    for (i, chars) in expanded_grid.iter().enumerate() {
        if chars.iter().all(|c| c == &'▒') {
            y_expand += 1;
        }
        for (j, c) in chars.iter().enumerate() {
            print!("{c}");
            if c == &'▒' {
                x_expand += 1
            }
            if c == &'#' {
                let ii64: i64 = i.try_into().unwrap();
                let ji64: i64 = j.try_into().unwrap();
                set.insert((
                    ii64 + y_expand * 1000000 - y_expand,
                    ji64 + x_expand * 1000000 - x_expand,
                ));
            }
        }
        x_expand = 0;

        println!();
    }
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
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 0)
    }
}
