pub fn star2(input: &str) -> usize {
    input.lines().collect::<Vec<&str>>().len()
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

