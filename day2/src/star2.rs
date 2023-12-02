use std::cmp::max;
pub fn star2(input: &str) -> u32 {
    let games = input.lines().map(parse_game);
    let powers = games.map(get_power);
    powers.sum()
}

fn parse_game(line: &str) -> Game {
    let games_str = line
        .split(": ")
        .skip(1)
        .next()
        .expect("string of all games");
    let reveals = games_str.split("; ").map(parse_reveal).collect();
    Game { reveals: reveals }
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

fn get_power(game: Game) -> u32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for reveal in game.reveals {
        max_red = max(max_red, reveal.red);
        max_green = max(max_green, reveal.green);
        max_blue = max(max_blue, reveal.blue);
    }
    max_blue * max_red * max_green
}

struct Pick {
    color: String,
    number: u32,
}

#[derive(Clone, Debug)]
struct Game {
    reveals: Vec<Reveal>,
}

#[derive(Clone, Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 2286)
    }
}
