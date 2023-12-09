pub fn star1(input: &str) -> i32 {
    let numbers = input.lines().map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()));
    numbers.map(find_nex_num).sum()
}

fn find_nex_num(numbers: impl Iterator<Item = i32>) -> i32 {
    let mut numbers_steps = vec![numbers.collect::<Vec<_>>()];
    while !numbers_steps.last().unwrap().iter().fold(true, |acc, n| n == &0 && acc) {
        let last_numbers = numbers_steps.last().unwrap();
        let mut new_numbers = vec![];
        for i in 0..last_numbers.len() - 1 {
            new_numbers.push(last_numbers[i + 1] - last_numbers[i])
        }
        numbers_steps.push(new_numbers);
    }
    for i in (0..numbers_steps.len()).rev() {
        if i == numbers_steps.len() - 1 {
            numbers_steps[i].push(0);
        } else {
            let last = numbers_steps[i].last().unwrap().clone();
            let last_last = numbers_steps[i + 1].last().unwrap().clone();
            numbers_steps[i].push(last + last_last);
        }
    }
    numbers_steps[0].last().unwrap().to_owned()
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 114)
    }

}
