use hashbrown::HashSet;
use itertools::Itertools;

#[aoc::puzzle("04.txt")]
#[aoc::assert("17782", "8477787")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let mut cards = input.split(line_ending).map(|line| {
        let mut numbers = line.split_whitespace().skip(2);
        let winning = numbers.take_while_ref(|c| c.as_bytes()!=b"|").collect::<HashSet<&str>>();
        (numbers.skip(1).fold(0, |acc, n| acc + winning.contains(n) as usize), 1)
    })
    .collect::<Vec<_>>();
    for i in 0..cards.len() { 
        for dx in i..i+cards[i].0 { cards[dx+1].1 += cards[i].1 }
    }
    cards.iter().fold((0,0), |acc, (c, m)| 
        (if *c>0 {acc.0 + (1<<(c-1))} else {acc.0}, acc.1 + m)
    )
}
