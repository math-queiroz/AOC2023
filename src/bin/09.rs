use itertools::Itertools;

#[aoc::puzzle("09.txt")]
#[aoc::assert("1904165718", "964")]
fn main(input: String, line_ending: &str) -> (isize, isize) {
    let series = input
        .split(line_ending)
        .map(|s| {
            let numbers = s
                .split_ascii_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect_vec();
            let mut l: Vec<_> = vec![numbers];
            loop {
                let item = l[l.len() - 1]
                    .windows(2)
                    .map(|n| n[1] - n[0])
                    .collect::<Vec<_>>();
                if item.iter().all(|n| *n == 0) {
                    break;
                } else {
                    l.push(item)
                }
            }
            l
        })
        .collect::<Vec<_>>();
    series
        .iter()
        .map(|s| {
            s.iter()
                .rev()
                .fold((0, 0), |(s1, s2), l| (s1 + l[l.len() - 1], l[0] - s2))
        })
        .fold((0, 0), |sum, s| (sum.0 + s.0, sum.1 + s.1))
}
