use hashbrown::HashMap;
use itertools::Itertools;

const CARDS_ORDER_PART1: &[u8] = "23456789TJQKA".as_bytes();
const CARDS_ORDER_PART2: &[u8] = "J23456789TQKA".as_bytes();

#[aoc::day(07, "Camel Cards")]
#[aoc::asserts("245794640", "247899149")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let hands = input.split(line_ending).map(|l| {
        let arr = l.split_whitespace().collect::<Vec<_>>();
        (arr[0].as_bytes(), arr[1].parse::<usize>().unwrap())
    });
    let ordered_hands = hands
        .clone()
        .sorted_by_cached_key(|(h, _)| qualify_hand(h, false));
    let ordered_hands_joker = hands.sorted_by_cached_key(|(h, _)| qualify_hand(h, true));
    (
        ordered_hands
            .enumerate()
            .fold(0, |acc, (i, (_, bet))| acc + bet * (i + 1)),
        ordered_hands_joker
            .enumerate()
            .fold(0, |acc, (i, (_, bet))| acc + bet * (i + 1)),
    )
}

fn qualify_hand(hand: &[u8], joker_rule: bool) -> (usize, usize) {
    let card_order = if joker_rule {
        CARDS_ORDER_PART2
    } else {
        CARDS_ORDER_PART1
    };
    let (mut card_count, value): (HashMap<u8, usize>, usize) =
        hand.iter()
            .fold((HashMap::new(), 0), |(mut map, value), c| {
                map.entry(*c).and_modify(|v| *v += 1).or_insert(1);
                (
                    map,
                    value * 13 + card_order.iter().position(|b| c == b).unwrap(),
                )
            });
    let joker_value = if joker_rule {
        card_count.remove(&b'J').unwrap_or(0)
    } else {
        0
    };
    let mut sorted_count = card_count.values().sorted().rev().take(2);
    let top_count = (
        sorted_count.next().unwrap_or(&0) + joker_value,
        sorted_count.next().unwrap_or(&0),
    );
    (
        match top_count {
            (5, 0) => 6,
            (4, 1) => 5,
            (3, 2) => 4,
            (3, 1) => 3,
            (2, 2) => 2,
            (2, 1) => 1,
            (1, 1) => 0,
            _ => unreachable!(),
        },
        value,
    )
}
