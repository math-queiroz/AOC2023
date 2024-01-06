use std::collections::VecDeque;

use hashbrown::HashSet;
use itertools::Itertools;

fn modulo(a: isize, b: isize) -> usize {
    let m = a % b;
    (match (m < 0, b < 0) {
        (false, _) => m,
        (true, false) => m + b,
        (true, true) => m - b,
    }) as usize
}

fn count_steps(map: &[&[u8]], (x, y): &(usize, usize), n: usize) -> usize {
    let (mut even_pos, mut seen) = (HashSet::new(), HashSet::new());
    let mut q = VecDeque::from_iter(vec![((*x as isize, *y as isize), n)]);
    while let Some((pos, step)) = q.pop_front() {
        if step % 2 == 0 {
            even_pos.insert(pos);
        }
        if step == 0 {
            continue;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (pos.0 + dx, pos.1 + dy);
            let (nxi, nyi) = (
                modulo(nx, map[0].len() as isize),
                modulo(ny, map.len() as isize),
            );
            if map[nyi][nxi] != b'#' && seen.insert((nx, ny)) {
                q.push_back(((nx, ny), step - 1));
            }
        }
    }
    even_pos.len()
}

fn fit_solve_quadratic(map: &[&[u8]], s_pos: &(usize, usize), n: usize) -> usize {
    let f = (0..3)
        .map(|t| count_steps(map, s_pos, n % map.len() + (map.len() * t)))
        .collect::<Vec<_>>();
    let df = [f[0], f[1] - f[0], f[2] - f[1]];
    let x = n / map.len();
    df[0] + df[1] * x + (x * (x - 1) / 2) * (df[2] - df[1])
}

#[aoc::day(21, "Step Counter")]
#[aoc::asserts("3594", "605247138198755")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let map = input
        .split(line_ending)
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    let s_pos = (0..map.len())
        .cartesian_product(0..map[0].len())
        .find(|&(y, x)| map[y][x] == b'S')
        .unwrap();
    (
        count_steps(&map, &s_pos, 64),
        fit_solve_quadratic(&map, &s_pos, 26501365),
    )
}
