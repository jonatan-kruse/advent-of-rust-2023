use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn star2(input: &str) -> usize {
    let list = input
        .lines()
        .map(|l| {
            let (text, numbs) = l.split_once(' ').unwrap();
            let numbs: Vec<usize> = numbs.split(',').map(|n| n.parse().unwrap()).collect();
            let repeated_text = (0..5).map(|_| text).collect::<Vec<&str>>().join("?");
            (repeated_text, numbs.repeat(5))
        })
        .collect::<Vec<_>>();

    // let list = input.lines().map(|l| {
    //     let (text, numbs) = l.split_once(' ').unwrap();
    //     (text, numbs.split(',').map(|n| n.parse().unwrap()).collect::<Vec<_>>())
    // }).collect::<Vec<_>>();
    list.par_iter()
        .map(|(t, n)| combinations(n.to_vec(), &t))
        .sum()
}

fn combinations(numbs: Vec<usize>, pattern: &str) -> usize {
    let char_slice: Vec<char> = pattern.chars().collect();
    let mut map = HashMap::new();
    let combs = get_combs(&numbs, &char_slice, &mut map);
    println!("combs: {}", combs);
    combs
}

fn get_combs(
    numbs: &[usize],
    pattern: &[char],
    map: &mut HashMap<(Vec<usize>, Vec<char>), usize>,
) -> usize {
    if numbs.is_empty() && pattern.is_empty() {
        return 1;
    }
    if let Some(num) = map.get(&(numbs.to_vec(), pattern.to_vec())) {
        return *num;
    }
    if pattern.is_empty() && !numbs.is_empty() {
        return 0;
    }


    let mut result = 0;

    if let Some((first, rest)) = numbs.split_first() {
        if number_fits_pattern(*first, &pattern) {
            if *first + 1 <= pattern.len() {
                result += get_combs(&rest.to_vec(), &pattern[(*first + 1)..], map);
            }
            if *first == pattern.len() {
                result += get_combs(&rest.to_vec(), &[], map);
            }
        }
    }

    if dot_fits_pattern(pattern) {
        result += get_combs(numbs, &pattern[1..], map);
    }

    map.insert((numbs.to_vec(), pattern.to_vec()), result);

    result
}

fn dot_fits_pattern(pattern: &[char]) -> bool {
    if pattern[0] == '#' {
        return false;
    }
    true
}

fn number_fits_pattern(num: usize, pattern: &[char]) -> bool {
    if num > pattern.len() {
        return false;
    }
    let mut count = 0;
    for &c in pattern.iter().take(num) {
        if c == '.' {
            return false;
        }
        if c == '?' || c == '#' {
            count += 1;
        }
    }
    if pattern.len() > num && pattern[num] == '#' {
        return false;
    }
    count >= num
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 525152)
    }
}
// 166239150459353 to big
// 160500973317706