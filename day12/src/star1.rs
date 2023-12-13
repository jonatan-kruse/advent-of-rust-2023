pub fn star1(input: &str) -> usize {
    let list = input.lines().map(|l| {
        let (text, numbs) = l.split_once(' ').unwrap();
        (text, numbs.split(',').map(|n| n.parse().unwrap()).collect())
    });
    list.map(|(t, n)| combinations(n, t.len()).filter(|c| is_valid(c, t)).count())
        .sum()
}

fn is_valid(test: &str, pattern: &str) -> bool {
    let boo = test.chars()
    .zip(pattern.chars())
    .all(|(tc, pc)| pc == '?' || tc == pc);
    boo
}

fn combinations(numbs: Vec<usize>, length: usize) -> impl Iterator<Item = String> {
    let combs = get_combs(numbs, length);
    combs.into_iter().map(|c| c.into_iter().collect())
}

fn get_combs(numbs: Vec<usize>, length: usize) -> Vec<Vec<char>> {
    if length == 0 {
        return vec![vec![]];
    }
     if numbs.is_empty() {
        return vec![vec!['.'; length]];
    }

    if numbs.iter().sum::<usize>() + numbs.len() - 1 > length {
        return vec![];
    }

    if numbs.len() == 1 && numbs[0] == length {
        return vec![vec!['#'; length]];
    }

    let mut result = Vec::new();

    // Generate combinations starting with '#'
    if let Some((first, rest)) = numbs.split_first() {
        if *first < length {
            let mut sub_combs = get_combs(rest.to_vec(), length - first - 1);
            for sub_comb in &mut sub_combs {
                let mut new_comb = vec!['#'; *first];
                new_comb.push('.'); // Ensure separation
                new_comb.append(sub_comb);
                result.push(new_comb);
            }
        }
    }

    // Generate combinations starting with '.'
    if length > 1 {
        let mut sub_combs = get_combs(numbs.clone(), length - 1);
        for sub_comb in sub_combs.iter_mut() {
            let mut new_comb = vec!['.'];
            new_comb.append(sub_comb);
            result.push(new_comb);
        }
    }

    result
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 21)
    }

}
