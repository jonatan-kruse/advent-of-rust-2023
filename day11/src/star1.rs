pub fn star1(input: &str) -> usize {
    input.lines().collect::<Vec<&str>>().len()
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 0)
    }

}
