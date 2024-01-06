#[aoc::day(02, "Cube Conundrum")]
#[aoc::asserts("2285", "77021")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let upper_limit = &[12, 13, 14];
    input
        .split(line_ending)
        .enumerate()
        .fold((0, 0), |acc, (i, l)| {
            let max_cubes = l
                .split_whitespace()
                .skip(2)
                .filter_map(|s| {
                    s.parse::<usize>()
                        .ok()
                        .or(s.bytes().next().map(|n| n as usize))
                })
                .collect::<Vec<_>>()
                .chunks(2)
                .fold([0; 3], |mut acc, pair| {
                    let pair = pair.to_vec();
                    match pair[1] as u8 {
                        b'r' => acc[0] = acc[0].max(pair[0]),
                        b'g' => acc[1] = acc[1].max(pair[0]),
                        b'b' => acc[2] = acc[2].max(pair[0]),
                        _ => panic!("Bad input"),
                    };
                    acc
                });
            let exceeds = max_cubes
                .iter()
                .enumerate()
                .all(|(i, c)| c <= &upper_limit[i]);
            (
                acc.0 + (1 + i) * exceeds as usize,
                acc.1 + max_cubes.iter().product::<usize>(),
            )
        })
}
