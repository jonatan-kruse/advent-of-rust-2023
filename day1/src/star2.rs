pub fn star2(input: &str) -> i32 {
    input
            .lines()
            .map(str_to_numbs)
            .map(|l| 10 * l[0] + l[l.len() - 1])
            .sum::<i32>()
}

fn str_to_numbs(str: &str) -> Vec<i32> {
    let mut temp: Vec<char> = vec![];
    let mut numbs: Vec<i32> = vec![];
    str.chars().for_each(|c| {
        if c.is_numeric() {
            numbs.push(c.to_string().parse().unwrap());
            temp = vec![];
        } else {
            temp.push(c);
            for (i, _) in temp.clone().into_iter().enumerate() {
                let s: String = temp.split_at(i).1.into_iter().collect();
                if let Some(a) = to_num(s.as_str()) {
                    numbs.push(a);
                    break;
                }
            }
        }
    });
    return numbs;
}

fn to_num(str: &str) -> Option<i32> {
    match str {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod test_star {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 281)
    }

}
