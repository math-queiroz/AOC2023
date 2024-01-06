use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

#[aoc::day(25, "Snowverload")]
#[aoc::asserts("495607", "🎄 Merry christmas!")]
fn main(input: String, line_ending: &str) -> (usize, &str) {
    let mut graph = input
        .split(line_ending)
        .map(|l| {
            let Some((k, v)) = l.split_once(": ") else { panic!("Bad input") };
            (k, v.split_whitespace().collect())
        })
        .collect::<HashMap<&str, Vec<&str>>>();
    for (k, v) in graph.clone() {
        for i in v {
            graph.entry(i).and_modify(|e| e.push(k)).or_insert(vec![k]);
        }
    }
    let mut edges = HashMap::<(&str, &str), usize>::new();
    for start in graph.keys() {
        let mut q = VecDeque::from_iter(vec![start]);
        let mut seen = HashSet::new();
        let mut track = HashMap::new();
        while let Some(n) = q.pop_front() {
            for nx in &graph[n] {
                if seen.insert(nx) {
                    q.push_back(nx);
                    track.insert(nx, n);
                }
            }
        }
        for mut prev in graph.keys() {
            while prev != start {
                let tmp = track[prev];
                *edges.entry((tmp.min(prev), tmp.max(prev))).or_insert(0) += 1;
                prev = &tmp;
            }
        }
    }
    let mut sorted_edges = edges.iter().collect::<Vec<_>>();
    sorted_edges.sort_by(|a, b| b.1.cmp(a.1));
    for ((n1, n2), _) in &sorted_edges[..3] {
        let p1 = graph[n1].iter().position(|s| s == n2).unwrap();
        let p2 = graph[n2].iter().position(|s| s == n1).unwrap();
        graph.get_mut(n1).unwrap().swap_remove(p1);
        graph.get_mut(n2).unwrap().swap_remove(p2);
    }
    let mut linked = HashSet::new();
    let mut q = VecDeque::from_iter(vec![graph.keys().next().unwrap()]);
    while let Some(n) = q.pop_front() {
        for nx in &graph[n] {
            if linked.insert(nx) {
                q.push_back(nx);
            }
        }
    }
    let product = linked.len() * (graph.len() - linked.len());
    (product, "🎄 Merry christmas!")
}
