pub fn star1(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let mut seeds: Vec<usize> = it.next().unwrap().split(':').skip(1).next().unwrap().split_whitespace().into_iter().map(|n| n.parse().unwrap()).collect();
    let maps:Vec<Vec<Vec<usize>>> = it.map(create_map).collect();
    for map in maps {
        let mut temp = vec![];
        for seed in &seeds {
            temp.push(get_value(map.clone(), seed))
        }
        seeds = temp;
    }
    seeds.iter().min().unwrap().to_owned()
}


fn create_map(input: &str) -> Vec<Vec<usize>> {
    input.lines().skip(1).map(|l| l.split_ascii_whitespace().into_iter().map(|n| n.parse().unwrap()).collect()).collect()
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
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 35)
    }

}
