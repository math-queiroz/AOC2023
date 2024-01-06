use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

#[aoc::day(23, "A Long Walk")]
#[aoc::asserts("2174", "6506")]
fn main(input: String, line_ending: &str) -> (usize, usize) {
    let map = input
        .split(line_ending)
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    let directed = parse(&map);
    let undirected = undirect(directed.clone());
    (
        dfs(&directed, &mut HashSet::new(), 0),
        dfs(&undirected, &mut HashSet::new(), 0),
    )
}

fn parse(map: &[&[u8]]) -> Vec<(usize, Vec<usize>)> {
    let mut seen = HashMap::<(isize, isize), usize>::new();
    let mut nodes = Vec::<(usize, Vec<usize>)>::new();
    let mut q = VecDeque::from_iter(vec![((1, 0), Some((0isize, 1isize)), 0)]);
    while let Some((mut pos, mut dir, prev)) = q.pop_front() {
        let (mut len, i) = (0, nodes.len());
        let p = (pos.0 + dir.unwrap().0, pos.1 + dir.unwrap().1);
        if seen.contains_key(&p) {
            nodes[prev].1.push(seen[&p]);
            continue;
        } else {
            seen.insert(p, i);
            if i > 0 {
                nodes[prev].1.push(i)
            }
        }
        let mut ignored_first = false;
        while let Some(d) = dir {
            (pos, dir) = ((pos.0 + d.0, pos.1 + d.1), None);
            for n in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                if n == (-d.0, -d.1) {
                    continue;
                }
                let (nx, ny) = ((pos.0 + n.0) as usize, (pos.1 + n.1) as usize);
                if nx >= map[0].len() || ny >= map.len() {
                    continue;
                }
                match (n, map[ny][nx]) {
                    (_, b'#') => continue,
                    (_, b'.') => dir = Some(n),
                    ((1, 0), b'<') | ((-1, 0), b'>') | ((0, 1), b'^') | ((0, -1), b'v') => {
                        if ignored_first {
                            continue;
                        } else {
                            ignored_first = true
                        }
                    }
                    _ => q.push_back(((nx as isize, ny as isize), Some(n), i)),
                }
            }
            len += 1;
        }
        nodes.push((len + 1, vec![]));
    }
    nodes
}

fn undirect(mut nodes: Vec<(usize, Vec<usize>)>) -> Vec<(usize, Vec<usize>)> {
    for (i, (_, conn)) in nodes.clone().iter().enumerate() {
        for c in conn {
            if !nodes[*c].1.contains(&i) {
                nodes[*c].1.push(i)
            }
        }
    }
    nodes
}

fn dfs(nodes: &[(usize, Vec<usize>)], seen: &mut HashSet<usize>, n: usize) -> usize {
    if n == (nodes.len() - 1) {
        return nodes[nodes.len() - 1].0 - 1;
    }
    let mut dist = 0;
    seen.insert(n);
    for nx in &nodes[n].1 {
        if !seen.contains(nx) {
            dist = dist.max(dfs(nodes, seen, *nx) + nodes[n].0);
        }
    }
    seen.remove(&n);
    dist
}
