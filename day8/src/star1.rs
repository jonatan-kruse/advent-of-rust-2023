use std::collections::HashMap;

pub fn star1(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let map = lines.skip(1).map(|line| {
        let binding = line.replace(" ", "");
        let mut it =binding.split('=');
        let from = it.next().unwrap().to_owned();
        let binding = it.next().unwrap().replace('(', "").replace(')', "");
        let mut it2 = binding.split(',');
        let to = (it2.next().unwrap().to_owned(), it2.next().unwrap().to_owned());
        (from, to)
    }).collect::<HashMap<_, _>>();
    
    let mut location = "AAA".to_string();
    let mut steps = 0;

    while location != "ZZZ" {
        let i = instructions[steps % instructions.len()];
        let options = map.get(&location.to_string()).unwrap();
        location = if i == 'L' {options.0.clone()} else {options.1.clone()};
        steps += 1
    }
steps
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 6)
    }

}
