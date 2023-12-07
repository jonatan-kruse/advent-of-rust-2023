use std::cmp::Ordering;

pub fn star1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(|line| {
        let mut it = line.split_whitespace();
        get_hand(it.next().unwrap().to_owned(), it.next().unwrap().parse().unwrap())
    }).collect();
    hands.sort_by(get_winning_hand);
    hands.iter().enumerate().map(|(i, hand)| hand.bid  * (i + 1)).sum()
}

fn get_hand(cards: String, bid: usize) -> Hand {
    let cards_v: Vec<usize> = cards.chars().map(to_value).collect();
    let hand_type = if is_of_a_kind(cards_v.clone(), 5) { 7 }
    else if is_of_a_kind(cards_v.clone(), 4) { 6 }
    else if is_full_house(cards_v.clone()) { 5 }
    else if is_of_a_kind(cards_v.clone(), 3) { 4 }
    else if is_two_pair(cards_v.clone()) { 3 }
    else if is_of_a_kind(cards_v.clone(), 2) { 2 }
    else { 1 };
    Hand { cards: cards_v, bid, hand_type }
}

fn get_winning_hand(hand1: &Hand, hand2: &Hand) -> Ordering {
    if hand1.hand_type > hand2.hand_type {
        return Ordering::Greater;
    } else if hand1.hand_type < hand2.hand_type {
        return Ordering::Less;
    } else {
        for i in 0..hand1.cards.len() {
            let card1 = hand1.cards[i];
            let card2 = hand2.cards[i];
            if card1 > card2 { return Ordering::Greater };
            if card1 < card2 { return Ordering::Less };
        }
    }
    Ordering::Equal
}

fn is_of_a_kind(cards: Vec<usize>, kind: usize) -> bool {
    let mut max_matches = 0;
    cards.iter().for_each(|c| {
        let matches = cards.iter().filter(|&x| x == c).count();
        if matches > max_matches { max_matches = matches }
    });
    max_matches == kind
}

fn is_full_house(cards: Vec<usize>) -> bool {
    let mut matches = vec![];
    cards.iter().for_each(|c| {
        let m = cards.iter().filter(|&x| x == c).count();
        matches.push(m)
    });
    matches.contains(&3) && matches.contains(&2)
}

fn is_two_pair(cards: Vec<usize>) -> bool {
    let mut matches = vec![];
    cards.iter().for_each(|c| {
        let m = cards.iter().filter(|&x| x == c).count();
        matches.push(m)
    });
    matches.iter().filter(|x| x == &&2).count() == 4
}

fn to_value(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        c => c.to_digit(10).unwrap().try_into().unwrap()
    }
}

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    hand_type: usize
}

#[cfg(test)]
mod test_star1 {
    use super::star1;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t1");
        let result = star1(test_input);
        assert_eq!(result, 6440)
    }

}
