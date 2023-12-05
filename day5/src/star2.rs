pub fn star2(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let seed_ranges: Vec<usize> = it.next().unwrap().split(':').skip(1).next().unwrap().split_whitespace().into_iter().map(|n| n.parse().unwrap()).collect();
    let mut seeds_candidates: Vec<usize> = vec![];
    let maps:Vec<Vec<Vec<usize>>> = it.map(create_map).collect();

    // Get all edge cases of all ranges moving from the last mapping backwards to the start this way we get a list of potential seeds instead of trying them all
    for i in (0..maps.len()).rev() {
        let new_candidates = get_map_candidates(maps[i].clone(), seeds_candidates.clone());
        seeds_candidates = new_candidates;

    }
    // Add the starting edge cases as well
    for i in 0..seed_ranges.len() - 1 {
        if i%2 == 0 {
            seeds_candidates.push(seed_ranges[i]);
            seeds_candidates.push(seed_ranges[i] + seed_ranges[i+1]);
        }
    }
    // Filter out all candidates that are not inside the given start ranges
    seeds_candidates = filter_candidates(seeds_candidates.clone(), seed_ranges.clone());
    
    // Push all candidates through all the maps to the bottom
    for map in maps {
        let mut temp = vec![];
        for seed in &seeds_candidates {
            temp.push(get_value(map.clone(), seed))
        }
        seeds_candidates = temp;
    }
    seeds_candidates.iter().min().unwrap().to_owned()
}

fn filter_candidates(candidates: Vec<usize>, ranges: Vec<usize>) -> Vec<usize> {
    candidates.iter().filter(|c| {
        for i in 0..ranges.len() - 1 {
            if i%2 == 0 && c >= &&ranges[i] && c <= &&(ranges[i + 1] + ranges[i]) {
                return true;
            }
        }
        return false;
    }).map(|c| c.to_owned()).collect()
}

fn create_map(input: &str) -> Vec<Vec<usize>> {
    input.lines().skip(1).map(|l| l.split_ascii_whitespace().into_iter().map(|n| n.parse().unwrap()).collect()).collect()
}

fn get_map_candidates(map: Vec<Vec<usize>>, values: Vec<usize>) -> Vec<usize> {
    let mut candidates: Vec<usize> = values.iter().map(|v| get_key(map.clone(), v)).collect();
    for range in map {
        candidates.push(range[1]);
        candidates.push(range[1] + range[2])
    }
    candidates
}

fn get_key(map: Vec<Vec<usize>>, value: &usize) -> usize {
    for range_data in map {
        let dest_start = range_data[0];
        let src_start = range_data[1];
        let range_len = range_data[2];
        if value >= &dest_start && value < &(dest_start + range_len) {
            return value - dest_start + src_start;
        }
    }
    return *value;
}

fn get_value(map: Vec<Vec<usize>>, key: &usize) -> usize {
    for range_data in map {
        let src_start = range_data[1];
        let dest_start = range_data[0];
        let range_len = range_data[2];
        if key >= &src_start && key < &(src_start + range_len) {
            return key - src_start + dest_start;
        }
    }
    return *key;
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 46)
    }
}

