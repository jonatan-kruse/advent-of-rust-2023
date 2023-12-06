use std::{iter::zip, usize};

pub fn star1(input: &str) -> usize {
    let mut lines = input.lines().map(|l| l.split_whitespace().skip(1).map(|num| num.parse::<usize>().unwrap()));
    let races: Vec<(usize, usize)> = zip(lines.next().unwrap(), lines.next().unwrap()).collect();
    races.iter().map(get_amount_of_wins).fold(1, |acc, num| acc * num)
}

fn get_amount_of_wins((time, dist): &(usize, usize)) -> usize {
    let variations = (0.try_into().unwrap()..time.to_owned()).map(|speed| {
        let travel_time = time - speed;
        travel_time * speed
    });
    variations.filter(|t| t > dist).count()
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 288)
    }

}
