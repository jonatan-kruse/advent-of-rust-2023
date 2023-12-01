pub fn star1() {
    println!("star1: {}", (include_str!("i1")
        .lines()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|l| format!("{}{}", l[0], l[l.len() - 1]).parse::<i32>().unwrap())
        .sum::<i32>()));
}
