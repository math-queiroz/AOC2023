use itertools::Itertools;

fn find_win_interval_length((time, distance): (usize, usize)) -> usize {
    (
        (((time as f64 + f64::sqrt((time * time - 4 * distance) as f64)) / 2f64).ceil() - 1f64) -
        (((time as f64 - f64::sqrt((time * time - 4 * distance) as f64)) / 2f64).floor() + 1f64)
    ) as usize + 1
}

#[aoc::puzzle("06.txt")]
#[aoc::assert("1731600", "40087680")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let (times, distances) = input
        .split(line_ending)
        .map(|l| {
            (
                l.split_whitespace()
                    .skip(1)
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
                l.bytes()
                    .skip(1)
                    .filter(|c| c.is_ascii_digit())
                    .fold(0, |acc, d| acc * 10 + (d - b'0') as usize),
            )
        })
        .collect_tuple()
        .unwrap();
    (
        times
            .0
            .into_iter()
            .zip(distances.0)
            .map(find_win_interval_length)
            .product(),
        find_win_interval_length((times.1, distances.1)),
    )
}
