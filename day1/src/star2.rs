pub fn star2() {
    println!(
        "star2: {}",
        (include_str!("i1")
            .lines()
            .map(str_to_numbs)
            .map(|l| format!("{}{}", l[0], l[l.len() - 1])
                .parse::<i32>()
                .unwrap())
            .sum::<i32>())
    );
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
                    temp = vec![];
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
