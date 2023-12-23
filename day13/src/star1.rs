pub fn star1(input: &str) -> usize {
    let squares = input.split("\n\n").map(|s| s.split("\n").map(|s|s.to_owned()).collect::<Vec<_>>());
    let transposed = squares.clone().map(|s| transpose(s));
    let combined = squares.zip(transposed);
    combined.map(|(s1, s2)| {
        if let Some(i) = get_square_score(s1) {
            i * 100
        } else if let Some(i) = get_square_score(s2) {
            i
        } else {
            panic!("No square found")
        }
    }).sum()
}

fn get_square_score(s: Vec<String>) -> Option<usize> {
    for i in 0..s.len() - 1 {
        let mut ii = i;
        let mut iii = i + 1;
            while s[ii] == s[iii] {
                if ii == 0 || iii == s.len() - 1 {
                    return Some(i + 1);
                }
                iii += 1;
                ii -= 1;
            }
    }
    None
}

fn transpose(s: Vec<String>) -> Vec<String> {
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
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 405)
    }

}
