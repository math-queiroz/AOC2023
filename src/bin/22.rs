use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

#[aoc::day(22, "Sand Slabs")]
#[aoc::asserts("522", "83519")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let mut bricks = input
        .split(line_ending)
        .map(|line| {
            line.split([',', '~'])
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();
    bricks.sort_by(|b1, b2| b1[2].cmp(&b2[2]));

    for i in 0..bricks.len() {
        let mut settle_z = 1;
        for j in 0..i {
            if overlaps(&bricks[i], &bricks[j]) {
                settle_z = settle_z.max(bricks[j][5] + 1);
            }
        }
        bricks[i][5] -= bricks[i][2] - settle_z;
        bricks[i][2] = settle_z;
    }

    let mut supports = HashMap::<usize, Vec<usize>>::new();
    let mut supp_by = HashMap::<usize, Vec<usize>>::new();
    for i in 0..bricks.len() {
        for j in i + 1..bricks.len() {
            if overlaps(&bricks[i], &bricks[j]) && bricks[i][5] == bricks[j][2] - 1 {
                supports.entry(i).or_default().push(j);
                supp_by.entry(j).or_default().push(i);
            }
        }
    }

    let safe = (0..bricks.len())
        .filter(|i| !supports.contains_key(i) || supports[i].iter().all(|j| supp_by[j].len() > 1))
        .count();

    let chain = (0..bricks.len())
        .map(|i| {
            if !supports.contains_key(&i) {
                return 0;
            }
            let deps = supports[&i]
                .clone()
                .into_iter()
                .filter(|j| supp_by[j].len() == 1);
            let (mut q, mut falling) = (
                VecDeque::from_iter(deps.clone()),
                HashSet::<usize>::from_iter(deps),
            );
            falling.insert(i);
            while let Some(j) = q.pop_front() {
                if !supports.contains_key(&j) {
                    continue;
                }
                for brick in &supports[&j] {
                    if falling.contains(brick) {
                        continue;
                    }
                    if supp_by[brick].iter().all(|b| falling.contains(b)) {
                        q.push_back(*brick);
                        falling.insert(*brick);
                    }
                }
            }
            falling.len() - 1
        })
        .sum::<usize>();

    (safe, chain)
}

fn overlaps(b1: &[usize], b2: &[usize]) -> bool {
    b1[0].max(b2[0]) <= b1[3].min(b2[3]) && b1[1].max(b2[1]) <= b1[4].min(b2[4])
}
