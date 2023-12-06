use aho_corasick::AhoCorasick;

#[aoc::puzzle("01.txt")]
#[aoc::assert("54338", "53389")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    input
        .split(line_ending)
        .map(|line| {
            let ac = AhoCorasick::new(vec![
                "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
                "six", "seven", "eight", "nine",
            ])
            .unwrap();
            let matches = ac
                .find_overlapping_iter(line)
                .map(|m| m.pattern().as_usize())
                .collect::<Vec<usize>>();
            return (
                matches
                    .iter()
                    .map(|n| *n + 1)
                    .filter(|n| *n < 10)
                    .collect::<Vec<usize>>(),
                matches.iter().map(|n| (n % 9) + 1).collect::<Vec<usize>>(),
            );
        })
        .map(|(n1, n2)| {
            (
                n1.last().unwrap() + n1.first().unwrap_or(n1.last().unwrap()) * 10,
                n2.last().unwrap() + n2.first().unwrap_or(n2.last().unwrap()) * 10,
            )
        })
        .fold((0, 0), |acc, (n1, n2)| (acc.0 + n1, acc.1 + n2))
}
