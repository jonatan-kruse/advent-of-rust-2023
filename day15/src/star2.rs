use std::collections::HashMap;

pub fn star2(input: &str) -> usize {
    let data = input.split(",").map(str_to_instruction);
    let mut hash_map: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
    for i in data {
        let mut vec = hash_map.get(&i.hash).unwrap_or(&vec![]).clone();
        if let Some(value) = i.value {
            let mut found = false;
            for j in 0..vec.len() {
                if vec[j].0 == i.str {
                    vec[j].1 = value;
                    found = true;
                }
            }
            if !found {
                vec.push((i.str, value));
            }
        } else {
            vec = vec.into_iter().filter(|v| v.0 != i.str).collect::<Vec<(String, usize)>>();
        }
        hash_map.insert(i.hash, vec);
    }
    hash_map.iter().map(calculate_score).sum()
}

fn calculate_score((b, list): (&usize, &Vec<(String, usize)>)) -> usize {
    list.iter().enumerate().map(|(i, (_, v))| (b + 1) * (i + 1) * v).sum()
}

fn str_to_instruction(input: &str) -> Instruction {
    let chars = input.chars();
    let str = chars.clone().take_while(|c| c != &'=' && c != &'-').collect::<String>();
    let hash = hash(str.chars().map(|c| c as usize).collect::<Vec<usize>>());
    let mut end = chars.skip_while(|c| c != &'=' && c != &'-');
    let value = if end.next() == Some('-') {
        None
    } else {
        let number = end.collect::<String>();
        Some(number.parse::<usize>().unwrap())
    };
    Instruction { str, hash, value }
}

struct Instruction {
    str: String,
    hash: usize,
    value: Option<usize>
}

fn hash(input: Vec<usize>) -> usize {
    input.iter().fold(0, |acc, i| ((acc + i) * 17) % 256)
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 145)
    }
}

