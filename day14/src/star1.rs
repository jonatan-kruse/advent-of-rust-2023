pub fn star1(input: &str) -> usize {
    let lines= input.lines();
    let mut cols = vec![Vec::new(); lines.clone().next().unwrap().len()];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            cols[i].push(c);
        }
    }
    cols.iter().map(move_stones_left).map(|col| calculate_weight(&col)).sum()
}

fn move_stones_left(col: &Vec<char>) -> Vec<char> {
    let mut new_col = col.clone();
    let mut i = 0;
    while i < new_col.len() {
        if new_col[i] == '.' {
           switch_stones(&mut new_col, i);
        }
        i += 1;
    }
    new_col
}

fn calculate_weight(col: &Vec<char>) -> usize {
    col.iter().rev().enumerate().map(|(i, c)| if *c == 'O' { i + 1 } else { 0 }).sum()
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
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 136)
    }

}
