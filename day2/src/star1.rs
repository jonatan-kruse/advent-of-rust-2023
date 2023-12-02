use std::cmp::max;
pub fn star1(input: &str) -> i32 {
    let ids = input.lines().map(parse_game).filter_map(|game| {
        if is_valid_game(game.clone()) {
            Some(game.id)
        } else {
            None
        }
    });
    ids.sum()
}

fn parse_game(line: &str) -> Game {
    let mut it = line.split(": ");
    let first = it.next().expect("a string");
    let id: i32 = first
        .split(' ')
        .last()
        .expect("string")
        .parse()
        .expect("number");
    let games_str = it.next().expect("string of all games");
    let reveals = games_str.split("; ").map(parse_reveal).collect();
    Game {
        id: id,
        reveals: reveals,
    }
}

fn parse_reveal(game_str: &str) -> Reveal {
    let picks = game_str.split(", ").map(|pick_str| {
        let mut pick_split = pick_str.split(" ");
        let number: u32 = pick_split.next().unwrap().parse().unwrap();
        let color = pick_split.next().unwrap();
        Pick {
            color: color.to_string(),
            number: number,
        }
    });

    let mut reveal = Reveal {
        red: 0,
        green: 0,
        blue: 0,
    };

    picks.for_each(|pick| match pick.color.as_str() {
        "red" => reveal.red = pick.number,
        "green" => reveal.green = pick.number,
        "blue" => reveal.blue = pick.number,
        _ => panic!("did not match any color"),
    });
    reveal
}

fn is_valid_game(game: Game) -> bool {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for reveal in game.clone().reveals {
        max_red = max(max_red, reveal.red);
        max_green = max(max_green, reveal.green);
        max_blue = max(max_blue, reveal.blue);
    }
    max_red <= 12 && max_green <= 13 && max_blue <= 14
}

struct Pick {
    color: String,
    number: u32,
}

#[derive(Clone, Debug)]
struct Game {
    id: i32,
    reveals: Vec<Reveal>,
}

#[derive(Clone, Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 8)
    }
}
