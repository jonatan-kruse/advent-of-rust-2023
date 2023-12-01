pub fn star1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|l| 10 * l[0].to_digit(10).unwrap() + l[l.len() - 1].to_digit(10).unwrap())
        .sum::<u32>()
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 142)
    }

}