use itertools::Itertools;
use std::cmp::{max, min};

#[aoc::day(05, "If You Give A Seed A Fertilizer")]
#[aoc::asserts("403695602", "219529182")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let double_line_ending = &line_ending.repeat(2);
    let seeds: Vec<usize> = input
        .split(line_ending)
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let categories: Vec<Vec<(usize, usize, usize)>> = input
        .split(double_line_ending)
        .skip(1)
        .map(|l| {
            l.split(line_ending)
                .skip(1)
                .map(|r| {
                    r.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect();
    (p1(seeds.clone(), &categories), p2(seeds, &categories))
}

fn p1(seeds: Vec<usize>, categories: &[Vec<(usize, usize, usize)>]) -> usize {
    seeds
        .iter()
        .map(|seed| {
            categories.iter().fold(*seed, |v, cat| {
                cat.iter()
                    .filter(|range| v >= range.1 && v < range.1 + range.2)
                    .fold(v, |acc, hit| acc + hit.0 - hit.1)
            })
        })
        .min()
        .unwrap()
}

fn p2(seeds: Vec<usize>, categories: &[Vec<(usize, usize, usize)>]) -> usize {
    let seed_ranges: Vec<(usize, usize)> = seeds.chunks(2).map(|s| (s[0], s[0] + s[1])).collect();
    categories
        .iter()
        .fold(seed_ranges, |seed_ranges, cat| {
            seed_ranges
                .iter()
                .flat_map(|seed_range| {
                    let mut mapped_dest = vec![];
                    let mut unmapped_dest = vec![seed_range.to_owned()];
                    for mapping in cat {
                        let mut temp_range = vec![];
                        let offset = mapping.0 - mapping.1;
                        for r in unmapped_dest {
                            let (pre, over, pos) = (
                                (r.0, min(mapping.1, r.1)),
                                (max(r.0, mapping.1), min(mapping.1 + mapping.2, r.1)),
                                (max(r.0, mapping.1 + mapping.2), r.1),
                            );
                            if pre.0 < pre.1 {
                                temp_range.push(pre);
                            }
                            if over.0 < over.1 {
                                mapped_dest.push((over.0 + offset, over.1 + offset));
                            }
                            if pos.0 < pos.1 {
                                temp_range.push(pos);
                            }
                        }
                        unmapped_dest = temp_range;
                    }
                    mapped_dest.extend(unmapped_dest);
                    mapped_dest
                })
                .collect()
        })
        .iter()
        .min_by_key(|(i, _)| *i)
        .unwrap()
        .0
}
