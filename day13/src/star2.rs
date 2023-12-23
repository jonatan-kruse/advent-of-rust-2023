pub fn star2(input: &str) -> usize {
    let squares = input
        .split("\n\n")
        .map(|s| s.split("\n").map(|s| s.to_owned()).collect::<Vec<_>>());
    let multi_flipped = squares.clone().map(|s| {
        let mut result = Vec::new();
        for i in 0..s.len() {
            for j in 0..s[0].len() {
                result.push(flip_one(s.clone(), i, j));
            }
        }
        result
    });
    let all_squares = multi_flipped.zip(squares);
    all_squares
        .map(|(squares, square)| {
            let original_score = calculate_both_scores(&square, None)[0].unwrap();
            squares
                .into_iter()
                .map(|s| calculate_both_scores(&s, Some(original_score)))
                .flatten()
                .map(|s| s.unwrap_or(original_score))
                .filter(|s| *s != original_score).max().unwrap()
        })
        .sum()
}

fn flip_one(sq: Vec<String>, row: usize, col: usize) -> Vec<String> {
    let mut result = sq.clone();
    let mut chars = result[row].chars().collect::<Vec<_>>();
    chars[col] = if chars[col] == '.' { '#' } else { '.' };
    result[row] = chars.into_iter().collect();
    result
}

fn calculate_both_scores(s: &Vec<String>, score: Option<usize>) -> Vec<Option<usize>> {
    let score1 = calculate_score(s, if let Some(score) = score { Some(score / 100) } else { None });
    let score2 = calculate_score(&transpose(s), score);
    if score1.is_some() && score2.is_some() {
        vec![Some(score1.unwrap() * 100), score2]
    } else if score1.is_some() {
        vec![Some(score1.unwrap() * 100)]
    } else if score2.is_some() {
        vec![score2]
    } else {
        vec![None]
    }
}

fn calculate_score(s: &Vec<String>, score: Option<usize>) -> Option<usize> {
    for i in 0..s.len() - 1 {
        let mut ii = i;
        let mut iii = i + 1;
        while s[ii] == s[iii] {
            if ii == 0 || iii == s.len() - 1 {
                if let Some(score) = score {
                    if i + 1 == score {
                        break;
                    }
                }
                return Some(i + 1);
            }
            iii += 1;
            ii -= 1;
        }
    }
    None
}

fn transpose(s: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..s[0].len() {
        let mut row = String::new();
        for j in 0..s.len() {
            row.push(s[j].chars().nth(i).unwrap());
        }
        result.push(row);
    }
    result
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 400)
    }
}


// To low: 21240
// wrong: 23574
// wrong: 30766
// correct: 31954
// To high: 38261
// To high: 45140
