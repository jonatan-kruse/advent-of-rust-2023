pub fn star2(input: &str) -> usize {
    let mut lines = input.lines().map(|l| l.replace(" ", "").split(":").skip(1).next().unwrap().parse::<usize>().unwrap());
    let race: (usize, usize) = (lines.next().unwrap(), lines.next().unwrap());
    get_amount_of_wins(&race)
}

fn get_amount_of_wins((time, dist): &(usize, usize)) -> usize {
    let variations = (0.try_into().unwrap()..time.to_owned()).map(|speed| {
        let travel_time = time - speed;
        travel_time * speed
    });
    variations.filter(|t| t > dist).count()
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 71503)
    }
}

