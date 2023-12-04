pub fn star1(input: &str) -> usize {
    let numbs: Vec<(Vec<usize>, Vec<usize>)> = input.lines().map(parse_input).collect();
    numbs.iter().map(get_points).sum()
}

fn parse_input(line: &str) -> (Vec<usize>, Vec<usize>) {
    let line2 = line.split(':').skip(1).next().unwrap();
    let mut it = line2.split('|');
    let win_numbs: Vec<usize> = it.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let my_numbs: Vec<usize> = it.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    return (win_numbs, my_numbs);
}

fn get_points(numbs: &(Vec<usize>, Vec<usize>)) -> usize {
    let winnings: Vec<&usize> = numbs.0.iter().filter_map(|num| numbs.1.iter().find(|n| &num == n)).collect();
    if winnings.len() == 0 { 0 } else {(2 as usize).pow((winnings.len() - 1) as u32)}
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 13)
    }

}
