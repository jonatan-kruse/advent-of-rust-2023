pub fn star2(input: &str) -> usize {
    let mut cards: Vec<Card> = input.lines().map(parse_card).collect();
    for i in 0..cards.len() {
        add_cards(i, &mut cards);
    }
    dbg!(cards.clone());
    cards.iter().map(|c| c.copies).sum()
}

fn parse_card(line: &str) -> Card {
    let line2 = line.split(':').skip(1).next().unwrap();
    let mut it2 = line2.split('|');
    let win_numbs: Vec<usize> = it2.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let my_numbs: Vec<usize> = it2.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    Card {
        copies: 1,
        numbers: (win_numbs, my_numbs)
    }
}

fn add_cards(c_index: usize, cards: &mut Vec<Card>) {
    let card = &cards[c_index];
    let copies = card.copies.clone();
    let winnings = card.numbers.0.iter().filter_map(|num| card.numbers.1.iter().find(|n| &num == n)).collect::<Vec<_>>().len();
    if winnings > 0 {
        for i in 1..=winnings {
            let change = &mut cards[c_index + i];
            change.copies += copies;
        }
    }
    
}

#[derive(Debug, Clone)]
struct Card {
    copies: usize,
    numbers: (Vec<usize>, Vec<usize>),
}

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 30)
    }
}

