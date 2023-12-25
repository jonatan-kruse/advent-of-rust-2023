pub fn star1(input: &str) -> usize {
    let data = input.split(",").map(|l | l.chars().map(|c| c as usize).collect::<Vec<usize>>());
    data.map(|l| hash(l)).sum()
}

fn hash(input: Vec<usize>) -> usize {
    input.iter().fold(0, |acc, i| ((acc + i) * 17) % 256)
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 1320)
    }

}
