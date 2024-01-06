use hashbrown::HashMap;
use std::collections::BinaryHeap;

#[aoc::day(17, "Clumsy Crucible")]
#[aoc::asserts("1039", "1201")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let map = input
        .split(line_ending)
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    (find_path(&map, 1, 3), find_path(&map, 4, 10))
}

fn find_path(map: &Vec<&[u8]>, min_steps: usize, max_steps: usize) -> usize {
    let mut seen = HashMap::new();
    let mut pq = BinaryHeap::new();
    pq.push((usize::MAX, (0, 0, 0, 0, 0)));
    while let Some((heat, (x, y, dx, dy, n))) = pq.pop() {
        if (x, y) == (map[0].len() - 1, map.len() - 1) && n >= min_steps {
            return usize::MAX - heat;
        }
        if seen.get(&(x, y, dx, dy, n)).is_some_and(|h| *h > heat) {
            continue;
        }
        for (ndx, ndy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (ndx, ndy) == (-dx, -dy) {
                continue;
            }
            let straight = (ndx, ndy) == (dx, dy);
            if straight {
                if n >= max_steps && (dx, dy) != (0, 0) {
                    continue;
                }
            } else if n < min_steps && (dx, dy) != (0, 0) {
                continue;
            }
            let nn = if straight { n + 1 } else { 1 };
            let nx = (x as isize + ndx) as usize;
            let ny = (y as isize + ndy) as usize;
            if nx >= map[0].len() || ny >= map.len() {
                continue;
            }
            let nheat = heat - (map[ny][nx] - b'0') as usize;
            let state = (nx, ny, ndx, ndy, nn);
            if seen.get(&state).map(|h| nheat > *h).unwrap_or(true) {
                seen.insert(state, nheat);
                pq.push((nheat, (nx, ny, ndx, ndy, nn)));
            }
        }
    }
    unreachable!()
}
